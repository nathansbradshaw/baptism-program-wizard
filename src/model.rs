use serde_json::Value;

/// Generates a small string enum with `as_str`/`parse`/`parse_or`, mirroring
/// the JS `sanitizeChoice(value, choices, fallback)` pattern used throughout
/// the original app.js for every constrained string field.
macro_rules! str_enum {
    ($name:ident { $($variant:ident => $s:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name { $($variant),+ }
        impl $name {
            pub fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $s),+ }
            }
            pub fn parse(s: &str) -> Option<Self> {
                match s { $($s => Some(Self::$variant),)+ _ => None }
            }
            pub fn parse_or(s: &str, fallback: Self) -> Self {
                Self::parse(s).unwrap_or(fallback)
            }
        }
    };
}

str_enum!(Size { Small => "small", Medium => "medium", Large => "large" });
str_enum!(Align { Left => "left", Center => "center", Right => "right" });
str_enum!(HeadingColor { Ink => "ink", Accent => "accent" });
str_enum!(TextStyle { Normal => "normal", Italic => "italic", Eyebrow => "eyebrow" });
str_enum!(ItemSize { Compact => "compact", Cozy => "cozy", Roomy => "roomy" });
str_enum!(ItemLineStyle { Underline => "underline", Dotted => "dotted", Plain => "plain", NoLine => "none" });
str_enum!(Columns { One => "1", Two => "2" });
str_enum!(Weight { Default => "default", Bold => "bold", Regular => "regular" });
str_enum!(Slant { Default => "default", Italic => "italic", Regular => "regular" });
str_enum!(FontOverride { Default => "default", Serif => "serif", Sans => "sans", Soft => "soft" });
str_enum!(FontFamily { Serif => "serif", Sans => "sans", Soft => "soft" });
str_enum!(Shape { Square => "square", Soft => "soft", Circle => "circle" });
str_enum!(DecorationStyle {
    Olive => "olive", Water => "water", Jordan => "jordan", Dove => "dove", Scriptures => "scriptures",
    Temple => "temple", TreeLife => "tree-life", Rays => "rays", Line => "line", Custom => "custom"
});

impl DecorationStyle {
    pub const ALL: [DecorationStyle; 10] = [
        DecorationStyle::Olive, DecorationStyle::Water, DecorationStyle::Jordan, DecorationStyle::Dove,
        DecorationStyle::Scriptures, DecorationStyle::Temple, DecorationStyle::TreeLife, DecorationStyle::Rays,
        DecorationStyle::Line, DecorationStyle::Custom,
    ];
    pub fn label(self) -> &'static str {
        match self {
            DecorationStyle::Olive => "Olive branch",
            DecorationStyle::Water => "Baptismal water",
            DecorationStyle::Jordan => "River Jordan",
            DecorationStyle::Dove => "Dove",
            DecorationStyle::Scriptures => "Open scriptures",
            DecorationStyle::Temple => "Temple",
            DecorationStyle::TreeLife => "Tree of Life",
            DecorationStyle::Rays => "Light rays",
            DecorationStyle::Line => "Simple divider",
            DecorationStyle::Custom => "Custom",
        }
    }
    /// Old saves used a different set of decoration names.
    fn parse_legacy_or(s: &str, fallback: DecorationStyle) -> DecorationStyle {
        let mapped = match s {
            "floral" => "olive",
            "waves" => "water",
            "dots" => "rays",
            "diamond" => "temple",
            other => other,
        };
        DecorationStyle::parse_or(mapped, fallback)
    }
}

impl FontFamily {
    pub fn css_stack(self) -> &'static str {
        match self {
            FontFamily::Serif => "Georgia, \"Times New Roman\", serif",
            FontFamily::Sans => "Arial, Helvetica, sans-serif",
            FontFamily::Soft => "\"Avenir Next\", Avenir, \"Trebuchet MS\", sans-serif",
        }
    }
}

impl FontOverride {
    pub fn css_stack(self) -> Option<&'static str> {
        match self {
            FontOverride::Default => None,
            FontOverride::Serif => Some(FontFamily::Serif.css_stack()),
            FontOverride::Sans => Some(FontFamily::Sans.css_stack()),
            FontOverride::Soft => Some(FontFamily::Soft.css_stack()),
        }
    }
}

// ---------------------------------------------------------------------------
// Pages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageId {
    Front,
    InsideLeft,
    InsideRight,
    Back,
}

impl PageId {
    pub const ALL: [PageId; 4] = [PageId::Front, PageId::InsideLeft, PageId::InsideRight, PageId::Back];

    pub fn as_str(self) -> &'static str {
        match self {
            PageId::Front => "front",
            PageId::InsideLeft => "inside-left",
            PageId::InsideRight => "inside-right",
            PageId::Back => "back",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            PageId::Front => "Front",
            PageId::InsideLeft => "Inside left",
            PageId::InsideRight => "Inside right",
            PageId::Back => "Back",
        }
    }

}

#[derive(Debug, Clone)]
pub struct Page {
    pub id: PageId,
    pub blocks: Vec<Block>,
}

// ---------------------------------------------------------------------------
// Typography (the shared "more styling" override block)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Typography {
    pub weight: Weight,
    pub slant: Slant,
    pub color: String,
    pub font: FontOverride,
}

impl Default for Typography {
    fn default() -> Self {
        Typography { weight: Weight::Default, slant: Slant::Default, color: String::new(), font: FontOverride::Default }
    }
}

// ---------------------------------------------------------------------------
// Block field structs (one per block type, matching app.js `block()` defaults)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct HeadingBlock { pub text: String, pub size: Size, pub align: Align, pub color: HeadingColor }
impl Default for HeadingBlock {
    fn default() -> Self { Self { text: "New heading".into(), size: Size::Medium, align: Align::Center, color: HeadingColor::Ink } }
}

#[derive(Debug, Clone)]
pub struct TextBlock { pub text: String, pub style: TextStyle, pub align: Align }
impl Default for TextBlock {
    fn default() -> Self { Self { text: "Add your message here.".into(), style: TextStyle::Normal, align: Align::Center } }
}

#[derive(Debug, Clone)]
pub struct ItemBlock { pub label: String, pub text: String, pub size: ItemSize, pub style: ItemLineStyle, pub align: Align }
impl Default for ItemBlock {
    fn default() -> Self {
        Self { label: "Program item".into(), text: "Name or details".into(), size: ItemSize::Cozy, style: ItemLineStyle::Underline, align: Align::Left }
    }
}

#[derive(Debug, Clone)]
pub struct CalloutBlock { pub title: String, pub subtitle: String, pub size: Size, pub align: Align }
impl Default for CalloutBlock {
    fn default() -> Self {
        Self { title: "Baptism of Isaac".into(), subtitle: "By (Name) | Witnesses: (Name) & (Name)".into(), size: Size::Medium, align: Align::Center }
    }
}

#[derive(Debug, Clone)]
pub struct HymnBlock { pub title: String, pub lyrics: String, pub size: Size, pub align: Align, pub lyrics_align: Align, pub columns: Columns }
impl Default for HymnBlock {
    fn default() -> Self {
        Self {
            title: "I Am a Child of God (CS pg. 2)".into(),
            lyrics: "1. I am a child of God, And he has sent me here,\nHas given me an earthly home, With parents kind and dear.\n\n[Chorus]\nLead me, guide me, walk beside me, Help me find the way.\nTeach me all that I must do, To live with him someday.".into(),
            size: Size::Medium,
            align: Align::Center,
            lyrics_align: Align::Left,
            columns: Columns::One,
        }
    }
}

str_enum!(QuoteKind { Scripture => "scripture", Quote => "quote" });

#[derive(Debug, Clone)]
pub struct QuoteBlock { pub text: String, pub citation: String, pub kind: QuoteKind, pub size: Size, pub align: Align }
impl Default for QuoteBlock {
    fn default() -> Self {
        Self {
            text: "And Jesus, when he was baptized, went up straightway out of the water.".into(),
            citation: "Matthew 3:16".into(),
            kind: QuoteKind::Scripture,
            size: Size::Medium,
            align: Align::Center,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownBlock { pub text: String, pub align: Align }
impl Default for MarkdownBlock {
    fn default() -> Self {
        Self {
            text: "Use **bold** and *italic* for emphasis.\n\nA blank line starts a new paragraph.\n\n- Put list items on their own lines\n- Separated from other text by a blank line".into(),
            align: Align::Left,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageBlock { pub data: String, pub art: String, pub size: Size, pub shape: Shape, pub caption: String }
impl Default for ImageBlock {
    fn default() -> Self { Self { data: String::new(), art: String::new(), size: Size::Medium, shape: Shape::Soft, caption: String::new() } }
}

#[derive(Debug, Clone)]
pub struct DecorationBlock { pub style: DecorationStyle, pub size: Size, pub data: String }
impl Default for DecorationBlock {
    fn default() -> Self { Self { style: DecorationStyle::Olive, size: Size::Medium, data: String::new() } }
}

#[derive(Debug, Clone)]
pub struct SpacerBlock { pub size: Size }
impl Default for SpacerBlock {
    fn default() -> Self { Self { size: Size::Medium } }
}

// ---------------------------------------------------------------------------
// Block
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind { Heading, Text, Item, Callout, Hymn, Quote, Markdown, Image, Decoration, Spacer }

impl BlockKind {
    pub const ALL: [BlockKind; 10] = [
        BlockKind::Heading, BlockKind::Text, BlockKind::Item, BlockKind::Callout, BlockKind::Hymn,
        BlockKind::Quote, BlockKind::Markdown, BlockKind::Image, BlockKind::Decoration, BlockKind::Spacer,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            BlockKind::Heading => "heading",
            BlockKind::Text => "text",
            BlockKind::Item => "item",
            BlockKind::Callout => "callout",
            BlockKind::Hymn => "hymn",
            BlockKind::Quote => "quote",
            BlockKind::Markdown => "markdown",
            BlockKind::Image => "image",
            BlockKind::Decoration => "decoration",
            BlockKind::Spacer => "spacer",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            BlockKind::Heading => "Heading",
            BlockKind::Text => "Text",
            BlockKind::Item => "Program item",
            BlockKind::Callout => "Callout section",
            BlockKind::Hymn => "Hymn",
            BlockKind::Quote => "Scripture / quote",
            BlockKind::Markdown => "Markdown text",
            BlockKind::Image => "Image",
            BlockKind::Decoration => "Decoration",
            BlockKind::Spacer => "Space",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.as_str() == s)
    }

    pub fn new_block(self) -> Block {
        let id = new_id();
        match self {
            BlockKind::Heading => Block::Heading { id, typography: Typography::default(), data: HeadingBlock::default() },
            BlockKind::Text => Block::Text { id, typography: Typography::default(), data: TextBlock::default() },
            BlockKind::Item => Block::Item { id, typography: Typography::default(), data: ItemBlock::default() },
            BlockKind::Callout => Block::Callout { id, typography: Typography::default(), data: CalloutBlock::default() },
            BlockKind::Hymn => Block::Hymn { id, typography: Typography::default(), data: HymnBlock::default() },
            BlockKind::Quote => Block::Quote { id, typography: Typography::default(), data: QuoteBlock::default() },
            BlockKind::Markdown => Block::Markdown { id, typography: Typography::default(), data: MarkdownBlock::default() },
            BlockKind::Image => Block::Image { id, data: ImageBlock::default() },
            BlockKind::Decoration => Block::Decoration { id, data: DecorationBlock::default() },
            BlockKind::Spacer => Block::Spacer { id, data: SpacerBlock::default() },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Heading { id: String, typography: Typography, data: HeadingBlock },
    Text { id: String, typography: Typography, data: TextBlock },
    Item { id: String, typography: Typography, data: ItemBlock },
    Callout { id: String, typography: Typography, data: CalloutBlock },
    Hymn { id: String, typography: Typography, data: HymnBlock },
    Quote { id: String, typography: Typography, data: QuoteBlock },
    Markdown { id: String, typography: Typography, data: MarkdownBlock },
    Image { id: String, data: ImageBlock },
    Decoration { id: String, data: DecorationBlock },
    Spacer { id: String, data: SpacerBlock },
}

impl Block {
    pub fn id(&self) -> &str {
        match self {
            Block::Heading { id, .. } | Block::Text { id, .. } | Block::Item { id, .. }
            | Block::Callout { id, .. } | Block::Hymn { id, .. } | Block::Quote { id, .. } | Block::Markdown { id, .. }
            | Block::Image { id, .. } | Block::Decoration { id, .. } | Block::Spacer { id, .. } => id,
        }
    }

    pub fn kind(&self) -> BlockKind {
        match self {
            Block::Heading { .. } => BlockKind::Heading,
            Block::Text { .. } => BlockKind::Text,
            Block::Item { .. } => BlockKind::Item,
            Block::Callout { .. } => BlockKind::Callout,
            Block::Hymn { .. } => BlockKind::Hymn,
            Block::Quote { .. } => BlockKind::Quote,
            Block::Markdown { .. } => BlockKind::Markdown,
            Block::Image { .. } => BlockKind::Image,
            Block::Decoration { .. } => BlockKind::Decoration,
            Block::Spacer { .. } => BlockKind::Spacer,
        }
    }

    pub fn typography(&self) -> Option<&Typography> {
        match self {
            Block::Heading { typography, .. } | Block::Text { typography, .. } | Block::Item { typography, .. }
            | Block::Callout { typography, .. } | Block::Hymn { typography, .. } | Block::Quote { typography, .. }
            | Block::Markdown { typography, .. } => Some(typography),
            Block::Image { .. } | Block::Decoration { .. } | Block::Spacer { .. } => None,
        }
    }

    pub fn typography_mut(&mut self) -> Option<&mut Typography> {
        match self {
            Block::Heading { typography, .. } | Block::Text { typography, .. } | Block::Item { typography, .. }
            | Block::Callout { typography, .. } | Block::Hymn { typography, .. } | Block::Quote { typography, .. }
            | Block::Markdown { typography, .. } => Some(typography),
            Block::Image { .. } | Block::Decoration { .. } | Block::Spacer { .. } => None,
        }
    }

    /// Mirrors `duplicateBlock`: a deep copy with a fresh id.
    pub fn duplicated(&self) -> Block {
        let mut copy = self.clone();
        let new_id = new_id();
        match &mut copy {
            Block::Heading { id, .. } | Block::Text { id, .. } | Block::Item { id, .. }
            | Block::Callout { id, .. } | Block::Hymn { id, .. } | Block::Quote { id, .. } | Block::Markdown { id, .. }
            | Block::Image { id, .. } | Block::Decoration { id, .. } | Block::Spacer { id, .. } => *id = new_id,
        }
        copy
    }
}

pub fn new_id() -> String {
    web_sys::window()
        .and_then(|w| w.crypto().ok())
        .map(|c| c.random_uuid())
        .unwrap_or_else(|| {
            let random = js_sys::Math::random();
            format!("block-{}-{:x}", js_sys::Date::now() as u64, (random * 1e16) as u64)
        })
}

pub fn is_image_data_url(s: &str) -> bool {
    s.starts_with("data:image/jpeg;base64,") || s.starts_with("data:image/png;base64,") || s.starts_with("data:image/webp;base64,")
}

fn is_hex_color(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.len() == 7 && bytes[0] == b'#' && bytes[1..].iter().all(|b| b.is_ascii_hexdigit())
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub paper: String,
    pub text: String,
    pub accent: String,
    pub font: FontFamily,
    pub monochrome: bool,
}

pub fn theme_preset(name: &str) -> Option<Theme> {
    let (paper, text, accent, font, monochrome) = match name {
        "monochrome" => ("#ffffff", "#111111", "#444444", FontFamily::Serif, true),
        "classic" => ("#fffdf8", "#20201e", "#708a7b", FontFamily::Serif, false),
        "sage" => ("#f5f4eb", "#28342d", "#7b927e", FontFamily::Soft, false),
        "blue" => ("#f4f9fb", "#183044", "#6f9eb6", FontFamily::Serif, false),
        "blush" => ("#fff7f4", "#412f2b", "#bf8177", FontFamily::Soft, false),
        "night" => ("#172735", "#f8f1e6", "#d7b77b", FontFamily::Serif, false),
        _ => return None,
    };
    Some(Theme { paper: paper.into(), text: text.into(), accent: accent.into(), font, monochrome })
}

pub const THEME_PRESET_NAMES: [&str; 6] = ["monochrome", "classic", "sage", "blue", "blush", "night"];

/// Mirrors the `elements.preset` "which preset matches the current theme" lookup in `updateThemeInputs`.
pub fn matching_theme_preset(theme: &Theme) -> Option<&'static str> {
    THEME_PRESET_NAMES.into_iter().find(|name| theme_preset(name).as_ref() == Some(theme))
}

// ---------------------------------------------------------------------------
// Built-in art
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtGroup { Lds, Human, Ai }

impl ArtGroup {
    pub const ALL: [ArtGroup; 3] = [ArtGroup::Lds, ArtGroup::Human, ArtGroup::Ai];
    pub fn label(self) -> &'static str {
        match self {
            ArtGroup::Lds => "Latter-day Saint photography",
            ArtGroup::Human => "Human-drawn Bible artwork",
            ArtGroup::Ai => "AI-generated artwork",
        }
    }
}

pub struct ArtItem {
    pub id: &'static str,
    pub group: ArtGroup,
    pub name: &'static str,
    pub src: &'static str,
    pub alt: &'static str,
}

pub const BUILT_IN_ART: &[ArtItem] = &[
    ArtItem { id: "lds-christus", group: ArtGroup::Lds, name: "Temple Square Christus", src: "static/art/lds-temple-square-christus.webp", alt: "Photograph of the Christus statue at the Temple Square visitors' center" },
    ArtItem { id: "baptism-of-christ", group: ArtGroup::Human, name: "Baptism of Christ", src: "static/art/baptism-of-christ.webp", alt: "Hand-drawn illustration of Jesus Christ after His baptism, with a dove above Him" },
    ArtItem { id: "christ-and-john", group: ArtGroup::Human, name: "Christ and John", src: "static/art/christ-and-john.webp", alt: "Hand-drawn illustration of Jesus Christ speaking with John the Baptist" },
    ArtItem { id: "good-shepherd", group: ArtGroup::Human, name: "The Good Shepherd", src: "static/art/good-shepherd.webp", alt: "Hand-drawn illustration of a shepherd guiding his flock" },
    ArtItem { id: "font-room", group: ArtGroup::Ai, name: "Baptismal font", src: "static/art/storybook-baptismal-font.webp", alt: "AI-generated illustration of a baptismal font room" },
    ArtItem { id: "christus-simple", group: ArtGroup::Ai, name: "Christus \u{b7} simplified", src: "static/art/storybook-christus-simplified.webp", alt: "Simplified AI-generated illustration based on the Temple Square Christus statue" },
    ArtItem { id: "christ-line", group: ArtGroup::Ai, name: "Christ \u{b7} welcome", src: "static/art/storybook-christ-open-arms.webp", alt: "AI-generated illustration of Jesus Christ with open arms" },
    ArtItem { id: "christ-color", group: ArtGroup::Ai, name: "Good Shepherd", src: "static/art/storybook-good-shepherd.webp", alt: "AI-generated illustration of Jesus Christ holding a lamb" },
    ArtItem { id: "baptism-river", group: ArtGroup::Ai, name: "Baptism \u{b7} river", src: "static/art/storybook-baptism-river.webp", alt: "AI-generated illustration of Jesus Christ and John the Baptist in a river" },
    ArtItem { id: "watercolor-baptism", group: ArtGroup::Ai, name: "Baptism of Jesus \u{b7} 1", src: "static/art/watercolor-baptism-of-jesus.webp", alt: "AI-generated illustration of Jesus Christ being baptized by John the Baptist" },
    ArtItem { id: "watercolor-lamb", group: ArtGroup::Ai, name: "Jesus with the lamb \u{b7} 1", src: "static/art/watercolor-jesus-with-lamb.webp", alt: "AI-generated illustration of Jesus Christ holding a lamb" },
    ArtItem { id: "watercolor-christus", group: ArtGroup::Ai, name: "Christus \u{b7} 1", src: "static/art/watercolor-christus.webp", alt: "AI-generated illustration of the Christus statue" },
    ArtItem { id: "watercolor-open-arms", group: ArtGroup::Ai, name: "Jesus \u{b7} welcome \u{b7} 1", src: "static/art/watercolor-jesus-open-arms.webp", alt: "AI-generated illustration of Jesus Christ with welcoming open arms" },
    ArtItem { id: "watercolor-temple", group: ArtGroup::Ai, name: "Temple \u{b7} 1", src: "static/art/watercolor-lds-temple.webp", alt: "AI-generated illustration of a Latter-day Saint temple" },
    ArtItem { id: "watercolor-font", group: ArtGroup::Ai, name: "Baptismal font \u{b7} 1", src: "static/art/watercolor-baptismal-font.webp", alt: "AI-generated illustration of a baptismal font" },
    ArtItem { id: "watercolor-waters-mormon", group: ArtGroup::Ai, name: "Waters of Mormon \u{b7} 1", src: "static/art/watercolor-waters-of-mormon.webp", alt: "AI-generated scene of Alma baptizing Helam at the Waters of Mormon" },
    ArtItem { id: "childrens-baptism", group: ArtGroup::Ai, name: "Baptism of Jesus \u{b7} 2", src: "static/art/lds-childrens-baptism-of-jesus.webp", alt: "AI-generated illustration of Jesus Christ being baptized by John the Baptist" },
    ArtItem { id: "childrens-lamb", group: ArtGroup::Ai, name: "Jesus with the lamb \u{b7} 2", src: "static/art/lds-childrens-jesus-with-lamb.webp", alt: "AI-generated illustration of Jesus Christ holding a lamb" },
    ArtItem { id: "childrens-christus", group: ArtGroup::Ai, name: "Christus \u{b7} 2", src: "static/art/lds-childrens-christus.webp", alt: "AI-generated illustration of the Christus statue" },
    ArtItem { id: "childrens-open-arms", group: ArtGroup::Ai, name: "Jesus \u{b7} welcome \u{b7} 2", src: "static/art/lds-childrens-jesus-open-arms.webp", alt: "AI-generated illustration of Jesus Christ with welcoming open arms" },
    ArtItem { id: "childrens-temple", group: ArtGroup::Ai, name: "Temple \u{b7} 2", src: "static/art/lds-childrens-temple.webp", alt: "AI-generated illustration of a Latter-day Saint temple" },
    ArtItem { id: "childrens-font", group: ArtGroup::Ai, name: "Baptismal font \u{b7} 2", src: "static/art/lds-childrens-baptismal-font.webp", alt: "AI-generated illustration of a baptismal font" },
    ArtItem { id: "childrens-waters-mormon", group: ArtGroup::Ai, name: "Waters of Mormon \u{b7} 2", src: "static/art/lds-childrens-waters-of-mormon.webp", alt: "AI-generated scene of Alma baptizing Helam at the Waters of Mormon" },
];

pub fn find_art(id: &str) -> Option<&'static ArtItem> {
    BUILT_IN_ART.iter().find(|a| a.id == id)
}

fn is_known_art_id(id: &str) -> bool {
    id.is_empty() || find_art(id).is_some()
}

// ---------------------------------------------------------------------------
// Document
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Document {
    pub theme: Theme,
    pub pages: Vec<Page>,
}

impl Document {
    pub fn page(&self, id: PageId) -> &Page {
        self.pages.iter().find(|p| p.id == id).expect("all four pages always present")
    }

    pub fn page_mut(&mut self, id: PageId) -> &mut Page {
        self.pages.iter_mut().find(|p| p.id == id).expect("all four pages always present")
    }
}

pub fn default_document() -> Document {
    Document { theme: theme_preset("classic").unwrap(), pages: content_template("service-one-hymn").unwrap() }
}

// ---------------------------------------------------------------------------
// Content templates
// ---------------------------------------------------------------------------

fn heading(text: &str, size: Size, align: Align) -> Block {
    BlockKind::Heading.new_block().with_heading(|b| { b.text = text.into(); b.size = size; b.align = align; })
}
fn text_b(text: &str, style: TextStyle, align: Align) -> Block {
    BlockKind::Text.new_block().with_text(|b| { b.text = text.into(); b.style = style; b.align = align; })
}
fn item(label: &str, text: &str) -> Block {
    BlockKind::Item.new_block().with_item(|b| { b.label = label.into(); b.text = text.into(); })
}
fn item_styled(label: &str, text: &str, size: ItemSize, style: ItemLineStyle) -> Block {
    BlockKind::Item.new_block().with_item(|b| { b.label = label.into(); b.text = text.into(); b.size = size; b.style = style; })
}
fn callout(title: &str, subtitle: &str, size: Size) -> Block {
    BlockKind::Callout.new_block().with_callout(|b| { b.title = title.into(); b.subtitle = subtitle.into(); b.size = size; })
}
fn hymn(title: &str, lyrics: &str, size: Size, align: Align, lyrics_align: Align) -> Block {
    BlockKind::Hymn.new_block().with_hymn(|b| { b.title = title.into(); b.lyrics = lyrics.into(); b.size = size; b.align = align; b.lyrics_align = lyrics_align; })
}
fn quote(text: &str, citation: &str, kind: QuoteKind, size: Size, align: Align) -> Block {
    BlockKind::Quote.new_block().with_quote(|b| { b.text = text.into(); b.citation = citation.into(); b.kind = kind; b.size = size; b.align = align; })
}
fn image(art: &str, size: Size, shape: Shape) -> Block {
    BlockKind::Image.new_block().with_image(|b| { b.art = art.into(); b.size = size; b.shape = shape; })
}
fn decoration(style: DecorationStyle, size: Size) -> Block {
    BlockKind::Decoration.new_block().with_decoration(|b| { b.style = style; b.size = size; })
}
fn spacer(size: Size) -> Block {
    BlockKind::Spacer.new_block().with_spacer(|b| { b.size = size; })
}

// Small mutation helpers so the template functions above can read as terse
// field overrides, mirroring `block(type, { ...overrides })` in app.js.
impl Block {
    fn with_heading(mut self, f: impl FnOnce(&mut HeadingBlock)) -> Self { if let Block::Heading { data, .. } = &mut self { f(data); } self }
    fn with_text(mut self, f: impl FnOnce(&mut TextBlock)) -> Self { if let Block::Text { data, .. } = &mut self { f(data); } self }
    fn with_item(mut self, f: impl FnOnce(&mut ItemBlock)) -> Self { if let Block::Item { data, .. } = &mut self { f(data); } self }
    fn with_callout(mut self, f: impl FnOnce(&mut CalloutBlock)) -> Self { if let Block::Callout { data, .. } = &mut self { f(data); } self }
    fn with_hymn(mut self, f: impl FnOnce(&mut HymnBlock)) -> Self { if let Block::Hymn { data, .. } = &mut self { f(data); } self }
    fn with_quote(mut self, f: impl FnOnce(&mut QuoteBlock)) -> Self { if let Block::Quote { data, .. } = &mut self { f(data); } self }
    fn with_image(mut self, f: impl FnOnce(&mut ImageBlock)) -> Self { if let Block::Image { data, .. } = &mut self { f(data); } self }
    fn with_decoration(mut self, f: impl FnOnce(&mut DecorationBlock)) -> Self { if let Block::Decoration { data, .. } = &mut self { f(data); } self }
    fn with_spacer(mut self, f: impl FnOnce(&mut SpacerBlock)) -> Self { if let Block::Spacer { data, .. } = &mut self { f(data); } self }
}

pub fn content_template(key: &str) -> Option<Vec<Page>> {
    match key {
        "child-same-day" => Some(build_child_same_day()),
        "child-later" => Some(build_child_later()),
        "convert" => Some(build_convert()),
        "multiple" => Some(build_multiple()),
        "service-one-hymn" => Some(one_page_service_template(ServiceLayout::OneLeft)),
        "service-two-hymns" => Some(one_page_service_template(ServiceLayout::TwoLeft)),
        "service-split-hymns" => Some(one_page_service_template(ServiceLayout::Split)),
        _ => None,
    }
}

enum ServiceLayout { OneLeft, TwoLeft, Split }

fn one_page_service_template(layout: ServiceLayout) -> Vec<Page> {
    let hymn_size = if matches!(layout, ServiceLayout::TwoLeft) { Size::Small } else { Size::Medium };
    let hymn_block = |title: &str| hymn(title, "Paste the hymn verses and chorus here.", hymn_size, Align::Center, Align::Left);

    let inside_left_blocks = match layout {
        ServiceLayout::TwoLeft => vec![hymn_block("Opening Hymn"), decoration(DecorationStyle::Line, Size::Small), hymn_block("Closing Hymn")],
        _ => vec![hymn_block("Opening Hymn")],
    };

    let back_blocks = if matches!(layout, ServiceLayout::Split) {
        vec![hymn_block("Closing Hymn")]
    } else {
        vec![
            spacer(Size::Medium),
            decoration(DecorationStyle::Olive, Size::Large),
            heading("Thank You for Joining Us", Size::Medium, Align::Center),
            text_b("We are grateful for your love and support on this special day.", TextStyle::Italic, Align::Center),
            quote("And Jesus, when he was baptized, went up straightway out of the water.", "Matthew 3:16", QuoteKind::Scripture, Size::Small, Align::Center),
        ]
    };

    vec![
        Page { id: PageId::Front, blocks: vec![
            text_b("BAPTISM OF", TextStyle::Eyebrow, Align::Center),
            heading("Name Placeholder", Size::Large, Align::Center),
            decoration(DecorationStyle::Line, Size::Small),
            image("baptism-river", Size::Large, Shape::Square),
            text_b("August 30, 2026 \u{b7} 5:00 PM", TextStyle::Normal, Align::Center),
            text_b("Ward or Stake Name\nCity, State", TextStyle::Italic, Align::Center),
            decoration(DecorationStyle::Olive, Size::Large),
        ]},
        Page { id: PageId::InsideLeft, blocks: inside_left_blocks },
        Page { id: PageId::InsideRight, blocks: vec![
            heading("Order of Service", Size::Medium, Align::Center),
            decoration(DecorationStyle::Temple, Size::Medium),
            item_styled("Presiding", "Name", ItemSize::Compact, ItemLineStyle::Dotted),
            item_styled("Conducting", "Name", ItemSize::Compact, ItemLineStyle::Dotted),
            item_styled("Pianist", "Name", ItemSize::Compact, ItemLineStyle::Dotted),
            item_styled("Chorister", "Name", ItemSize::Compact, ItemLineStyle::Dotted),
            decoration(DecorationStyle::Line, Size::Small),
            item_styled("Opening Hymn", "Hymn title and number", ItemSize::Compact, ItemLineStyle::Plain),
            item_styled("Opening Prayer", "Name", ItemSize::Compact, ItemLineStyle::Plain),
            item_styled("Talk on Baptism", "Name", ItemSize::Compact, ItemLineStyle::Plain),
            callout("Baptism of Name", "Performed by Name | Witnesses: Name & Name", Size::Small),
            item_styled("Talk on the Holy Ghost", "Name", ItemSize::Compact, ItemLineStyle::Plain),
            callout("Confirmation", "Performed by Name", Size::Small),
            item_styled("Welcome", "Name", ItemSize::Compact, ItemLineStyle::Plain),
            item_styled("Closing Hymn", "Hymn title and number", ItemSize::Compact, ItemLineStyle::Plain),
            item_styled("Closing Prayer", "Name", ItemSize::Compact, ItemLineStyle::Plain),
        ]},
        Page { id: PageId::Back, blocks: back_blocks },
    ]
}

fn order_of_service_left() -> Vec<Block> {
    vec![
        heading("Order of Service", Size::Medium, Align::Center),
        decoration(DecorationStyle::Line, Size::Medium),
        item("Presiding", ""),
        item("Conducting", ""),
        item("Pianist", ""),
        item("Chorister", ""),
        decoration(DecorationStyle::Line, Size::Small),
        item("Welcome", ""),
        item("Opening Hymn", "When I Am Baptized"),
        item("Opening Prayer", ""),
        item("Talk on Baptism", ""),
    ]
}

fn thank_you_back(gratitude: &str, scripture: &str, reference: &str) -> Vec<Block> {
    vec![
        spacer(Size::Medium),
        decoration(DecorationStyle::Olive, Size::Large),
        heading("Thank You for Joining Us", Size::Medium, Align::Center),
        text_b(gratitude, TextStyle::Italic, Align::Center),
        decoration(DecorationStyle::Line, Size::Small),
        text_b(scripture, TextStyle::Italic, Align::Center),
        text_b(reference, TextStyle::Italic, Align::Center),
    ]
}

fn build_child_same_day() -> Vec<Page> {
    vec![
        Page { id: PageId::Front, blocks: vec![
            text_b("BAPTISM OF", TextStyle::Eyebrow, Align::Center),
            heading("Name Placeholder", Size::Large, Align::Center),
            decoration(DecorationStyle::Line, Size::Small),
            image("baptism-of-christ", Size::Medium, Shape::Square),
            text_b("August 30, 2026 \u{b7} 10:00 AM", TextStyle::Normal, Align::Center),
            text_b("Ward or Stake Name\nCity, State", TextStyle::Italic, Align::Center),
            spacer(Size::Small),
            text_b("A covenant to follow Jesus Christ", TextStyle::Italic, Align::Center),
        ]},
        Page { id: PageId::InsideLeft, blocks: order_of_service_left() },
        Page { id: PageId::InsideRight, blocks: vec![
            heading("Baptismal Ordinance", Size::Small, Align::Center),
            item("Performed by", ""),
            item("Witnessed by", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Talk on the Holy Ghost", ""),
            heading("Confirmation", Size::Small, Align::Center),
            item("Performed by", ""),
            item("Primary Welcome", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Closing Hymn", "I Am a Child of God"),
            item("Closing Prayer", ""),
            item("Refreshments", "Please join us afterward"),
        ]},
        Page { id: PageId::Back, blocks: thank_you_back(
            "We are grateful for your love and support on this special day.",
            "Willing to bear one another's burdens, to mourn with those that mourn, and to stand as witnesses of God at all times.",
            "\u{2014} Mosiah 18:8\u{2013}9",
        )},
    ]
}

fn build_child_later() -> Vec<Page> {
    vec![
        Page { id: PageId::Front, blocks: vec![
            text_b("BAPTISM OF", TextStyle::Eyebrow, Align::Center),
            heading("Name Placeholder", Size::Large, Align::Center),
            decoration(DecorationStyle::Line, Size::Small),
            image("baptism-river", Size::Medium, Shape::Square),
            text_b("August 30, 2026 \u{b7} 10:00 AM", TextStyle::Normal, Align::Center),
            text_b("Ward or Stake Name\nCity, State", TextStyle::Italic, Align::Center),
            spacer(Size::Small),
            text_b("A covenant to follow Jesus Christ", TextStyle::Italic, Align::Center),
        ]},
        Page { id: PageId::InsideLeft, blocks: order_of_service_left() },
        Page { id: PageId::InsideRight, blocks: vec![
            heading("Baptismal Ordinance", Size::Small, Align::Center),
            item("Performed by", ""),
            item("Witnessed by", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Testimonies", ""),
            item("Closing Hymn", "I Am a Child of God"),
            item("Closing Prayer", ""),
            item("Refreshments", "Please join us afterward"),
            text_b("Confirmation will take place during sacrament meeting.", TextStyle::Italic, Align::Center),
        ]},
        Page { id: PageId::Back, blocks: thank_you_back(
            "We are grateful for your love and support on this special day.",
            "And Jesus, when he was baptized, went up straightway out of the water: and, lo, the heavens were opened unto him, and he saw the Spirit of God descending like a dove, and lighting upon him.",
            "\u{2014} Matthew 3:16",
        )},
    ]
}

fn build_convert() -> Vec<Page> {
    vec![
        Page { id: PageId::Front, blocks: vec![
            text_b("BAPTISM OF", TextStyle::Eyebrow, Align::Center),
            heading("Name Placeholder", Size::Large, Align::Center),
            decoration(DecorationStyle::Line, Size::Small),
            image("christ-and-john", Size::Medium, Shape::Square),
            text_b("August 30, 2026 \u{b7} 10:00 AM", TextStyle::Normal, Align::Center),
            text_b("Ward or Stake Name\nCity, State", TextStyle::Italic, Align::Center),
            spacer(Size::Small),
            text_b("A new beginning in the gospel of Jesus Christ", TextStyle::Italic, Align::Center),
        ]},
        Page { id: PageId::InsideLeft, blocks: vec![
            heading("Order of Service", Size::Medium, Align::Center),
            decoration(DecorationStyle::Line, Size::Medium),
            item("Presiding", ""),
            item("Conducting", ""),
            item("Pianist", ""),
            item("Chorister", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Welcome", "Ward Mission Leader"),
            item("Opening Hymn", ""),
            item("Invocation", "Missionary"),
            item("Convert's Testimony", ""),
            item("Talk on Baptism", "Friend"),
        ]},
        Page { id: PageId::InsideRight, blocks: vec![
            heading("Baptismal Ordinance", Size::Small, Align::Center),
            item("Performed by", "Missionary"),
            item("Witnessed by", "Missionary, Friend"),
            decoration(DecorationStyle::Line, Size::Small),
            item("Talk on the Holy Ghost", "Missionary"),
            heading("Confirmation", Size::Small, Align::Center),
            item("Performed by", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Bishopric Welcome", "Bishop"),
            item("Closing Hymn", ""),
            item("Benediction", "Missionary"),
            item("Refreshments", "Please join us afterward"),
        ]},
        Page { id: PageId::Back, blocks: vec![
            spacer(Size::Medium),
            decoration(DecorationStyle::Olive, Size::Large),
            heading("Thank You for Joining Us", Size::Medium, Align::Center),
            item("Taught by", "Elder/Sister Name, Elder/Sister Name"),
            text_b("We are grateful for your love and support on this special day.", TextStyle::Italic, Align::Center),
            decoration(DecorationStyle::Line, Size::Small),
            text_b("Now faith is the assurance of things hoped for, the evidence of things not seen.", TextStyle::Italic, Align::Center),
            text_b("\u{2014} Hebrews 11:1", TextStyle::Italic, Align::Center),
        ]},
    ]
}

fn build_multiple() -> Vec<Page> {
    vec![
        Page { id: PageId::Front, blocks: vec![
            text_b("BAPTISM PROGRAM", TextStyle::Eyebrow, Align::Center),
            heading("Baptism Program", Size::Large, Align::Center),
            decoration(DecorationStyle::Line, Size::Small),
            image("baptism-of-christ", Size::Medium, Shape::Square),
            text_b("August 30, 2026 \u{b7} 10:00 AM", TextStyle::Normal, Align::Center),
            text_b("Ward or Stake Name\nCity, State", TextStyle::Italic, Align::Center),
            item("Being Baptized", "Name One, Name Two, Name Three"),
            text_b("A covenant to follow Jesus Christ", TextStyle::Italic, Align::Center),
        ]},
        Page { id: PageId::InsideLeft, blocks: order_of_service_left() },
        Page { id: PageId::InsideRight, blocks: vec![
            heading("Baptismal Ordinances", Size::Small, Align::Center),
            item("Performed by", ""),
            item("Witnessed by", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Talk on the Holy Ghost", ""),
            heading("Confirmations", Size::Small, Align::Center),
            item("Performed by", ""),
            decoration(DecorationStyle::Line, Size::Small),
            item("Closing Remarks", ""),
            item("Closing Hymn", "I Am a Child of God"),
            item("Closing Prayer", ""),
            item("Refreshments", "Please join us afterward"),
        ]},
        Page { id: PageId::Back, blocks: thank_you_back(
            "We are grateful for your love and support as we celebrate these baptisms.",
            "Witnessing before the church that they have truly repented of all their sins, and are willing to take upon them the name of Jesus Christ, having a determination to serve him to the end.",
            "\u{2014} Doctrine and Covenants 20:37",
        )},
    ]
}

// ---------------------------------------------------------------------------
// Sanitizing untrusted JSON (draft restore + loaded save files)
// ---------------------------------------------------------------------------

fn sanitize_text(v: Option<&Value>, fallback: &str) -> String {
    match v.and_then(|v| v.as_str()) {
        Some(s) => s.chars().take(10_000).collect(),
        None => fallback.to_string(),
    }
}

fn sanitize_color(v: Option<&str>, fallback: &str) -> String {
    match v {
        Some(s) if is_hex_color(s) => s.to_string(),
        _ => fallback.to_string(),
    }
}

fn get_str<'a>(obj: &'a serde_json::Map<String, Value>, key: &str) -> Option<&'a str> {
    obj.get(key).and_then(|v| v.as_str())
}

fn sanitize_typography(v: Option<&Value>) -> Typography {
    let obj = v.and_then(|v| v.as_object());
    let empty = serde_json::Map::new();
    let obj = obj.unwrap_or(&empty);
    Typography {
        weight: get_str(obj, "weight").map(|s| Weight::parse_or(s, Weight::Default)).unwrap_or(Weight::Default),
        slant: get_str(obj, "slant").map(|s| Slant::parse_or(s, Slant::Default)).unwrap_or(Slant::Default),
        color: sanitize_color(get_str(obj, "color"), ""),
        font: get_str(obj, "font").map(|s| FontOverride::parse_or(s, FontOverride::Default)).unwrap_or(FontOverride::Default),
    }
}

pub(crate) fn sanitize_block(v: &Value) -> Option<Block> {
    let obj = v.as_object()?;
    let kind = BlockKind::parse(get_str(obj, "type")?)?;
    let id: String = sanitize_text(obj.get("id"), &new_id()).chars().take(100).collect();
    let typography = sanitize_typography(obj.get("typography"));

    Some(match kind {
        BlockKind::Heading => {
            let d = HeadingBlock::default();
            Block::Heading { id, typography, data: HeadingBlock {
                text: sanitize_text(obj.get("text"), &d.text),
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
                color: get_str(obj, "color").map(|s| HeadingColor::parse_or(s, d.color)).unwrap_or(d.color),
            }}
        }
        BlockKind::Text => {
            let d = TextBlock::default();
            Block::Text { id, typography, data: TextBlock {
                text: sanitize_text(obj.get("text"), &d.text),
                style: get_str(obj, "style").map(|s| TextStyle::parse_or(s, d.style)).unwrap_or(d.style),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
            }}
        }
        BlockKind::Item => {
            let d = ItemBlock::default();
            Block::Item { id, typography, data: ItemBlock {
                label: sanitize_text(obj.get("label"), &d.label),
                text: sanitize_text(obj.get("text"), &d.text),
                size: get_str(obj, "size").map(|s| ItemSize::parse_or(s, d.size)).unwrap_or(d.size),
                style: get_str(obj, "style").map(|s| ItemLineStyle::parse_or(s, d.style)).unwrap_or(d.style),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
            }}
        }
        BlockKind::Callout => {
            let d = CalloutBlock::default();
            Block::Callout { id, typography, data: CalloutBlock {
                title: sanitize_text(obj.get("title"), &d.title),
                subtitle: sanitize_text(obj.get("subtitle"), &d.subtitle),
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
            }}
        }
        BlockKind::Hymn => {
            let d = HymnBlock::default();
            Block::Hymn { id, typography, data: HymnBlock {
                title: sanitize_text(obj.get("title"), &d.title),
                lyrics: sanitize_text(obj.get("lyrics"), &d.lyrics),
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
                lyrics_align: get_str(obj, "lyricsAlign").map(|s| Align::parse_or(s, d.lyrics_align)).unwrap_or(d.lyrics_align),
                columns: get_str(obj, "columns").map(|s| Columns::parse_or(s, d.columns)).unwrap_or(d.columns),
            }}
        }
        BlockKind::Quote => {
            let d = QuoteBlock::default();
            Block::Quote { id, typography, data: QuoteBlock {
                text: sanitize_text(obj.get("text"), &d.text),
                citation: sanitize_text(obj.get("citation"), &d.citation),
                kind: get_str(obj, "kind").map(|s| QuoteKind::parse_or(s, d.kind)).unwrap_or(d.kind),
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
            }}
        }
        BlockKind::Markdown => {
            let d = MarkdownBlock::default();
            Block::Markdown { id, typography, data: MarkdownBlock {
                text: sanitize_text(obj.get("text"), &d.text),
                align: get_str(obj, "align").map(|s| Align::parse_or(s, d.align)).unwrap_or(d.align),
            }}
        }
        BlockKind::Image => {
            let art = get_str(obj, "art").filter(|s| is_known_art_id(s)).unwrap_or("").to_string();
            let raw_data = get_str(obj, "data").unwrap_or("");
            let data = if !art.is_empty() { String::new() } else if is_image_data_url(raw_data) { raw_data.to_string() } else { String::new() };
            let d = ImageBlock::default();
            Block::Image { id, data: ImageBlock {
                art,
                data,
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
                shape: get_str(obj, "shape").map(|s| Shape::parse_or(s, d.shape)).unwrap_or(d.shape),
                caption: sanitize_text(obj.get("caption"), ""),
            }}
        }
        BlockKind::Decoration => {
            let d = DecorationBlock::default();
            let raw_data = get_str(obj, "data").unwrap_or("");
            Block::Decoration { id, data: DecorationBlock {
                style: get_str(obj, "style").map(|s| DecorationStyle::parse_legacy_or(s, d.style)).unwrap_or(d.style),
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
                data: if is_image_data_url(raw_data) { raw_data.to_string() } else { String::new() },
            }}
        }
        BlockKind::Spacer => {
            let d = SpacerBlock::default();
            Block::Spacer { id, data: SpacerBlock {
                size: get_str(obj, "size").map(|s| Size::parse_or(s, d.size)).unwrap_or(d.size),
            }}
        }
    })
}

/// Mirrors `sanitizeDocument`: tolerantly rebuild a valid Document from
/// arbitrary parsed JSON (sessionStorage draft, or a loaded save file).
pub fn sanitize_document(v: &Value) -> Document {
    let obj = v.as_object();
    let theme_obj = obj.and_then(|o| o.get("theme")).and_then(|v| v.as_object());
    let classic = theme_preset("classic").unwrap();
    let color = |key: &str, fallback: &str| -> String {
        sanitize_color(theme_obj.and_then(|o| o.get(key)).and_then(|v| v.as_str()), fallback)
    };
    let theme = Theme {
        paper: color("paper", &classic.paper),
        text: color("text", &classic.text),
        accent: color("accent", &classic.accent),
        font: theme_obj.and_then(|o| o.get("font")).and_then(|v| v.as_str()).map(|s| FontFamily::parse_or(s, FontFamily::Serif)).unwrap_or(FontFamily::Serif),
        monochrome: theme_obj.and_then(|o| o.get("monochrome")).and_then(|v| v.as_bool()).unwrap_or(false),
    };

    let candidate_pages = obj.and_then(|o| o.get("pages")).and_then(|v| v.as_array());
    let pages = PageId::ALL.iter().map(|&pid| {
        let source = candidate_pages.and_then(|pages| {
            pages.iter().find(|p| p.get("id").and_then(|v| v.as_str()) == Some(pid.as_str()))
        });
        let blocks = source
            .and_then(|p| p.get("blocks"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().take(100).filter_map(sanitize_block).collect())
            .unwrap_or_default();
        Page { id: pid, blocks }
    }).collect();

    Document { theme, pages }
}

/// Mirrors `legacyDocument`: upgrades a version-1 flat-field save into the
/// modern block-based shape.
pub fn legacy_document(fields: &Value) -> Document {
    let obj = fields.as_object();
    let empty = serde_json::Map::new();
    let obj = obj.unwrap_or(&empty);
    let text = |key: &str| -> String { sanitize_text(obj.get(key), "") };

    let mut doc = default_document();

    let mut front = vec![
        BlockKind::Decoration.new_block().with_decoration(|b| { b.style = DecorationStyle::Olive; b.size = Size::Medium; }),
        text_b("BAPTISM PROGRAM", TextStyle::Eyebrow, Align::Center),
        heading(&{ let n = text("name"); if n.is_empty() { "Name Placeholder".to_string() } else { n } }, Size::Large, Align::Center),
        decoration(DecorationStyle::Line, Size::Medium),
        text_b(&[text("date"), text("time")].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" \u{b7} "), TextStyle::Normal, Align::Center),
        text_b(&[text("ward"), text("location")].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("\n"), TextStyle::Italic, Align::Center),
    ];
    let front_image = text("front_image");
    if is_image_data_url(&front_image) {
        front.push(BlockKind::Image.new_block().with_image(|b| { b.data = front_image; b.size = Size::Large; }));
    }
    doc.page_mut(PageId::Front).blocks = front;

    let mut left = vec![heading("Order of Service", Size::Medium, Align::Center), decoration(DecorationStyle::Line, Size::Medium)];
    for (label, key) in [
        ("Welcome", "welcome"), ("Opening hymn", "opening_hymn"), ("Opening prayer", "opening_prayer"),
        ("Talk on baptism", "talk_baptism"), ("Baptism ordinance", "baptized_by"),
    ] {
        left.push(item(label, &text(key)));
    }
    let inside_left_image = text("inside_left_image");
    if is_image_data_url(&inside_left_image) {
        left.push(BlockKind::Image.new_block().with_image(|b| { b.data = inside_left_image; }));
    }
    doc.page_mut(PageId::InsideLeft).blocks = left;

    let mut right = vec![heading("Order of Service", Size::Medium, Align::Center), decoration(DecorationStyle::Line, Size::Medium)];
    for (label, key) in [
        ("Talk on the Holy Ghost", "talk_holy_ghost"), ("Confirmation", "confirmed_by"), ("Testimonies", "testimonies"),
        ("Closing hymn", "closing_hymn"), ("Closing prayer", "closing_prayer"), ("Refreshments", "refreshments"),
    ] {
        right.push(item(label, &text(key)));
    }
    let scripture_text = text("scripture_text");
    if !scripture_text.is_empty() {
        let reference = text("scripture_reference");
        let full = if reference.is_empty() { scripture_text } else { format!("{scripture_text}\n\u{2014} {reference}") };
        right.push(text_b(&full, TextStyle::Italic, Align::Center));
    }
    let inside_right_image = text("inside_right_image");
    if is_image_data_url(&inside_right_image) {
        right.push(BlockKind::Image.new_block().with_image(|b| { b.data = inside_right_image; }));
    }
    doc.page_mut(PageId::InsideRight).blocks = right;

    let mut back = Vec::new();
    let hymn_one_title = text("hymn_one_title");
    if !hymn_one_title.is_empty() { back.push(heading(&hymn_one_title, Size::Small, Align::Center)); }
    let hymn_one_lyrics = text("hymn_one_lyrics");
    if !hymn_one_lyrics.is_empty() { back.push(text_b(&hymn_one_lyrics, TextStyle::Normal, Align::Left)); }
    let hymn_two_title = text("hymn_two_title");
    if !hymn_two_title.is_empty() { back.push(heading(&hymn_two_title, Size::Small, Align::Center)); }
    let hymn_two_lyrics = text("hymn_two_lyrics");
    if !hymn_two_lyrics.is_empty() { back.push(text_b(&hymn_two_lyrics, TextStyle::Normal, Align::Left)); }
    let back_image = text("back_image");
    if is_image_data_url(&back_image) {
        back.push(BlockKind::Image.new_block().with_image(|b| { b.data = back_image; }));
    }
    doc.page_mut(PageId::Back).blocks = back;

    doc
}

// ---------------------------------------------------------------------------
// Serializing to JSON (for download-save and sessionStorage draft)
// ---------------------------------------------------------------------------

fn typography_json(t: &Typography) -> Value {
    serde_json::json!({
        "weight": t.weight.as_str(),
        "slant": t.slant.as_str(),
        "color": t.color,
        "font": t.font.as_str(),
    })
}

pub fn block_to_json(block: &Block) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("id".into(), Value::String(block.id().into()));
    map.insert("type".into(), Value::String(block.kind().as_str().into()));
    if let Some(typography) = block.typography() {
        map.insert("typography".into(), typography_json(typography));
    }
    match block {
        Block::Heading { data, .. } => {
            map.insert("text".into(), data.text.clone().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("align".into(), data.align.as_str().into());
            map.insert("color".into(), data.color.as_str().into());
        }
        Block::Text { data, .. } => {
            map.insert("text".into(), data.text.clone().into());
            map.insert("style".into(), data.style.as_str().into());
            map.insert("align".into(), data.align.as_str().into());
        }
        Block::Item { data, .. } => {
            map.insert("label".into(), data.label.clone().into());
            map.insert("text".into(), data.text.clone().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("style".into(), data.style.as_str().into());
            map.insert("align".into(), data.align.as_str().into());
        }
        Block::Callout { data, .. } => {
            map.insert("title".into(), data.title.clone().into());
            map.insert("subtitle".into(), data.subtitle.clone().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("align".into(), data.align.as_str().into());
        }
        Block::Hymn { data, .. } => {
            map.insert("title".into(), data.title.clone().into());
            map.insert("lyrics".into(), data.lyrics.clone().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("align".into(), data.align.as_str().into());
            map.insert("lyricsAlign".into(), data.lyrics_align.as_str().into());
            map.insert("columns".into(), data.columns.as_str().into());
        }
        Block::Quote { data, .. } => {
            map.insert("text".into(), data.text.clone().into());
            map.insert("citation".into(), data.citation.clone().into());
            map.insert("kind".into(), data.kind.as_str().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("align".into(), data.align.as_str().into());
        }
        Block::Markdown { data, .. } => {
            map.insert("text".into(), data.text.clone().into());
            map.insert("align".into(), data.align.as_str().into());
        }
        Block::Image { data, .. } => {
            map.insert("data".into(), data.data.clone().into());
            map.insert("art".into(), data.art.clone().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("shape".into(), data.shape.as_str().into());
            map.insert("caption".into(), data.caption.clone().into());
        }
        Block::Decoration { data, .. } => {
            map.insert("style".into(), data.style.as_str().into());
            map.insert("size".into(), data.size.as_str().into());
            map.insert("data".into(), data.data.clone().into());
        }
        Block::Spacer { data, .. } => {
            map.insert("size".into(), data.size.as_str().into());
        }
    }
    Value::Object(map)
}

pub fn document_to_json(doc: &Document) -> Value {
    serde_json::json!({
        "theme": {
            "paper": doc.theme.paper,
            "text": doc.theme.text,
            "accent": doc.theme.accent,
            "font": doc.theme.font.as_str(),
            "monochrome": doc.theme.monochrome,
        },
        "pages": doc.pages.iter().map(|p| serde_json::json!({
            "id": p.id.as_str(),
            "blocks": p.blocks.iter().map(block_to_json).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}
