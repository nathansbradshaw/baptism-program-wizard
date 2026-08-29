(() => {
  const pageDefinitions = [
    { id: "front", label: "Front" },
    { id: "inside-left", label: "Inside left" },
    { id: "inside-right", label: "Inside right" },
    { id: "back", label: "Back" }
  ];

  const themes = {
    monochrome: { paper: "#ffffff", text: "#111111", accent: "#444444", font: "serif", monochrome: true },
    classic: { paper: "#fffdf8", text: "#20201e", accent: "#708a7b", font: "serif", monochrome: false },
    sage: { paper: "#f5f4eb", text: "#28342d", accent: "#7b927e", font: "soft", monochrome: false },
    blue: { paper: "#f4f9fb", text: "#183044", accent: "#6f9eb6", font: "serif", monochrome: false },
    blush: { paper: "#fff7f4", text: "#412f2b", accent: "#bf8177", font: "soft", monochrome: false },
    night: { paper: "#172735", text: "#f8f1e6", accent: "#d7b77b", font: "serif", monochrome: false }
  };

  const blockNames = {
    heading: "Heading",
    text: "Text",
    item: "Program item",
    callout: "Callout section",
    hymn: "Hymn",
    quote: "Scripture / quote",
    markdown: "Markdown text",
    image: "Image",
    decoration: "Decoration",
    spacer: "Space"
  };

  const decorationOptions = [
    ["olive", "Olive branch"],
    ["water", "Baptismal water"],
    ["jordan", "River Jordan"],
    ["dove", "Dove"],
    ["scriptures", "Open scriptures"],
    ["temple", "Temple"],
    ["tree-life", "Tree of Life"],
    ["rays", "Light rays"],
    ["line", "Simple divider"],
    ["custom", "Custom"]
  ];
  const decorationStyles = decorationOptions.map(([value]) => value);

  const builtInArt = [
    { id: "lds-christus", group: "lds", name: "Temple Square Christus", src: "static/art/lds-temple-square-christus.webp", alt: "Photograph of the Christus statue at the Temple Square visitors' center" },
    { id: "baptism-of-christ", group: "human", name: "Baptism of Christ", src: "static/art/baptism-of-christ.webp", alt: "Hand-drawn illustration of Jesus Christ after His baptism, with a dove above Him" },
    { id: "christ-and-john", group: "human", name: "Christ and John", src: "static/art/christ-and-john.webp", alt: "Hand-drawn illustration of Jesus Christ speaking with John the Baptist" },
    { id: "good-shepherd", group: "human", name: "The Good Shepherd", src: "static/art/good-shepherd.webp", alt: "Hand-drawn illustration of a shepherd guiding his flock" },
    { id: "font-room", group: "ai", name: "Baptismal font", src: "static/art/storybook-baptismal-font.webp", alt: "AI-generated illustration of a baptismal font room" },
    { id: "christus-simple", group: "ai", name: "Christus · simplified", src: "static/art/storybook-christus-simplified.webp", alt: "Simplified AI-generated illustration based on the Temple Square Christus statue" },
    { id: "christ-line", group: "ai", name: "Christ · welcome", src: "static/art/storybook-christ-open-arms.webp", alt: "AI-generated illustration of Jesus Christ with open arms" },
    { id: "christ-color", group: "ai", name: "Good Shepherd", src: "static/art/storybook-good-shepherd.webp", alt: "AI-generated illustration of Jesus Christ holding a lamb" },
    { id: "baptism-river", group: "ai", name: "Baptism · river", src: "static/art/storybook-baptism-river.webp", alt: "AI-generated illustration of Jesus Christ and John the Baptist in a river" },
    { id: "watercolor-baptism", group: "ai", name: "Baptism of Jesus · 1", src: "static/art/watercolor-baptism-of-jesus.webp", alt: "AI-generated illustration of Jesus Christ being baptized by John the Baptist" },
    { id: "watercolor-lamb", group: "ai", name: "Jesus with the lamb · 1", src: "static/art/watercolor-jesus-with-lamb.webp", alt: "AI-generated illustration of Jesus Christ holding a lamb" },
    { id: "watercolor-christus", group: "ai", name: "Christus · 1", src: "static/art/watercolor-christus.webp", alt: "AI-generated illustration of the Christus statue" },
    { id: "watercolor-open-arms", group: "ai", name: "Jesus · welcome · 1", src: "static/art/watercolor-jesus-open-arms.webp", alt: "AI-generated illustration of Jesus Christ with welcoming open arms" },
    { id: "watercolor-temple", group: "ai", name: "Temple · 1", src: "static/art/watercolor-lds-temple.webp", alt: "AI-generated illustration of a Latter-day Saint temple" },
    { id: "watercolor-font", group: "ai", name: "Baptismal font · 1", src: "static/art/watercolor-baptismal-font.webp", alt: "AI-generated illustration of a baptismal font" },
    { id: "watercolor-waters-mormon", group: "ai", name: "Waters of Mormon · 1", src: "static/art/watercolor-waters-of-mormon.webp", alt: "AI-generated scene of Alma baptizing Helam at the Waters of Mormon" },
    { id: "childrens-baptism", group: "ai", name: "Baptism of Jesus · 2", src: "static/art/lds-childrens-baptism-of-jesus.webp", alt: "AI-generated illustration of Jesus Christ being baptized by John the Baptist" },
    { id: "childrens-lamb", group: "ai", name: "Jesus with the lamb · 2", src: "static/art/lds-childrens-jesus-with-lamb.webp", alt: "AI-generated illustration of Jesus Christ holding a lamb" },
    { id: "childrens-christus", group: "ai", name: "Christus · 2", src: "static/art/lds-childrens-christus.webp", alt: "AI-generated illustration of the Christus statue" },
    { id: "childrens-open-arms", group: "ai", name: "Jesus · welcome · 2", src: "static/art/lds-childrens-jesus-open-arms.webp", alt: "AI-generated illustration of Jesus Christ with welcoming open arms" },
    { id: "childrens-temple", group: "ai", name: "Temple · 2", src: "static/art/lds-childrens-temple.webp", alt: "AI-generated illustration of a Latter-day Saint temple" },
    { id: "childrens-font", group: "ai", name: "Baptismal font · 2", src: "static/art/lds-childrens-baptismal-font.webp", alt: "AI-generated illustration of a baptismal font" },
    { id: "childrens-waters-mormon", group: "ai", name: "Waters of Mormon · 2", src: "static/art/lds-childrens-waters-of-mormon.webp", alt: "AI-generated scene of Alma baptizing Helam at the Waters of Mormon" }
  ];
  const builtInArtIds = ["", ...builtInArt.map(({ id: artId }) => artId)];

  const imagePattern = /^data:image\/(?:jpeg|png|webp);base64,/;
  const draftKey = "baptism-program-document-v2";
  const pageClipboardStorageKey = "baptism-program-page-clipboard-v1";
  const pageClipboardDatabase = "baptism-program-shared-clipboard";
  let selectedPage = "front";
  let focusMode = false;
  let previewZoom = 1;
  let pageClipboard = null;
  let highlightedBlockId = "";
  let saveTimer;
  let statusTimer;

  const elements = {
    pageTabs: document.querySelector("#page-tabs"),
    blockList: document.querySelector("#block-list"),
    livePreview: document.querySelector("#live-preview"),
    toggleFocus: document.querySelector("#toggle-focus"),
    pageLabel: document.querySelector("#current-page-label"),
    elementCount: document.querySelector("#element-count"),
    newBlockType: document.querySelector("#new-block-type"),
    addBlock: document.querySelector("#add-block"),
    contentTemplate: document.querySelector("#content-template"),
    preset: document.querySelector("#theme-preset"),
    paper: document.querySelector("#paper-color"),
    text: document.querySelector("#text-color"),
    accent: document.querySelector("#accent-color"),
    font: document.querySelector("#font-family"),
    monochrome: document.querySelector("#monochrome-images"),
    status: document.querySelector("#save-status"),
    load: document.querySelector("#load-save"),
    copyPage: document.querySelector("#copy-page"),
    pastePage: document.querySelector("#paste-page"),
    zoomOut: document.querySelector("#zoom-out"),
    zoomIn: document.querySelector("#zoom-in"),
    zoomValue: document.querySelector("#zoom-value")
  };

  function id() {
    return globalThis.crypto?.randomUUID?.() || `block-${Date.now()}-${Math.random().toString(16).slice(2)}`;
  }

  const typographyBlockTypes = ["heading", "text", "item", "callout", "hymn", "quote", "markdown"];
  const fontStacks = {
    serif: 'Georgia, "Times New Roman", serif',
    sans: "Arial, Helvetica, sans-serif",
    soft: '"Avenir Next", Avenir, "Trebuchet MS", sans-serif'
  };

  function defaultTypography() {
    return { weight: "default", slant: "default", color: "", font: "default" };
  }

  function block(type, values = {}) {
    const defaults = {
      heading: { text: "New heading", size: "medium", align: "center", color: "ink" },
      text: { text: "Add your message here.", style: "normal", align: "center" },
      item: { label: "Program item", text: "Name or details", size: "cozy", style: "underline", align: "left" },
      callout: { title: "Baptism of Isaac", subtitle: "By (Name) | Witnesses: (Name) & (Name)", size: "medium", align: "center" },
      hymn: {
        title: "I Am a Child of God (CS pg. 2)",
        lyrics: "1. I am a child of God, And he has sent me here,\nHas given me an earthly home, With parents kind and dear.\n\n[Chorus]\nLead me, guide me, walk beside me, Help me find the way.\nTeach me all that I must do, To live with him someday.",
        size: "medium",
        align: "center",
        lyricsAlign: "left",
        columns: "1"
      },
      quote: {
        text: "And Jesus, when he was baptized, went up straightway out of the water.",
        citation: "Matthew 3:16",
        kind: "scripture",
        size: "medium",
        align: "center"
      },
      markdown: { text: "Use **bold** and *italic* for emphasis.\n\nA blank line starts a new paragraph.\n\n- Put list items on their own lines\n- Separated from other text by a blank line", align: "left" },
      image: { data: "", art: "", size: "medium", shape: "soft", caption: "" },
      decoration: { style: "olive", size: "medium", data: "" },
      spacer: { size: "medium" }
    };
    const merged = { id: id(), type, ...defaults[type], ...values };
    if (typographyBlockTypes.includes(type) && !merged.typography) merged.typography = defaultTypography();
    return merged;
  }

  function onePageServiceTemplate(layout) {
    const hymn = (title) => block("hymn", {
      title,
      lyrics: "Paste the hymn verses and chorus here.",
      size: layout === "two-left" ? "small" : "medium",
      align: "center",
      lyricsAlign: "left"
    });
    const insideLeftBlocks = layout === "two-left"
      ? [hymn("Opening Hymn"), block("decoration", { style: "line", size: "small" }), hymn("Closing Hymn")]
      : [hymn("Opening Hymn")];
    const backBlocks = layout === "split"
      ? [hymn("Closing Hymn")]
      : [
          block("spacer", { size: "medium" }),
          block("decoration", { style: "olive", size: "large" }),
          block("heading", { text: "Thank You for Joining Us", size: "medium", align: "center" }),
          block("text", { text: "We are grateful for your love and support on this special day.", style: "italic", align: "center" }),
          block("quote", {
            text: "And Jesus, when he was baptized, went up straightway out of the water.",
            citation: "Matthew 3:16",
            kind: "scripture",
            size: "small",
            align: "center"
          })
        ];

    return [
      {
        id: "front",
        blocks: [
          block("text", { text: "BAPTISM OF", style: "eyebrow", align: "center" }),
          block("heading", { text: "Name Placeholder", size: "large", align: "center" }),
          block("decoration", { style: "line", size: "small" }),
          block("image", { art: "baptism-river", size: "large", shape: "square" }),
          block("text", { text: "August 30, 2026 · 5:00 PM", align: "center" }),
          block("text", { text: "Ward or Stake Name\nCity, State", style: "italic", align: "center" }),
          block("decoration", { style: "olive", size: "large" })
        ]
      },
      { id: "inside-left", blocks: insideLeftBlocks },
      {
        id: "inside-right",
        blocks: [
          block("heading", { text: "Order of Service", size: "medium", align: "center" }),
          block("decoration", { style: "temple", size: "medium" }),
          block("item", { label: "Presiding", text: "Name", size: "compact", style: "dotted" }),
          block("item", { label: "Conducting", text: "Name", size: "compact", style: "dotted" }),
          block("item", { label: "Pianist", text: "Name", size: "compact", style: "dotted" }),
          block("item", { label: "Chorister", text: "Name", size: "compact", style: "dotted" }),
          block("decoration", { style: "line", size: "small" }),
          block("item", { label: "Opening Hymn", text: "Hymn title and number", size: "compact", style: "plain" }),
          block("item", { label: "Opening Prayer", text: "Name", size: "compact", style: "plain" }),
          block("item", { label: "Talk on Baptism", text: "Name", size: "compact", style: "plain" }),
          block("callout", { title: "Baptism of Name", subtitle: "Performed by Name | Witnesses: Name & Name", size: "small" }),
          block("item", { label: "Talk on the Holy Ghost", text: "Name", size: "compact", style: "plain" }),
          block("callout", { title: "Confirmation", subtitle: "Performed by Name", size: "small" }),
          block("item", { label: "Welcome", text: "Name", size: "compact", style: "plain" }),
          block("item", { label: "Closing Hymn", text: "Hymn title and number", size: "compact", style: "plain" }),
          block("item", { label: "Closing Prayer", text: "Name", size: "compact", style: "plain" })
        ]
      },
      { id: "back", blocks: backBlocks }
    ];
  }

  const contentTemplates = {
    "child-same-day": {
      label: "Child baptism + confirmation",
      build: () => [
        {
          id: "front",
          blocks: [
            block("text", { text: "BAPTISM OF", style: "eyebrow", align: "center" }),
            block("heading", { text: "Name Placeholder", size: "large", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("image", { art: "baptism-of-christ", size: "medium", shape: "square" }),
            block("text", { text: "August 30, 2026 · 10:00 AM", align: "center" }),
            block("text", { text: "Ward or Stake Name\nCity, State", style: "italic", align: "center" }),
            block("spacer", { size: "small" }),
            block("text", { text: "A covenant to follow Jesus Christ", style: "italic", align: "center" })
          ]
        },
        {
          id: "inside-left",
          blocks: [
            block("heading", { text: "Order of Service", size: "medium", align: "center" }),
            block("decoration", { style: "line", size: "medium" }),
            block("item", { label: "Presiding", text: "" }),
            block("item", { label: "Conducting", text: "" }),
            block("item", { label: "Pianist", text: "" }),
            block("item", { label: "Chorister", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Welcome", text: "" }),
            block("item", { label: "Opening Hymn", text: "When I Am Baptized" }),
            block("item", { label: "Opening Prayer", text: "" }),
            block("item", { label: "Talk on Baptism", text: "" })
          ]
        },
        {
          id: "inside-right",
          blocks: [
            block("heading", { text: "Baptismal Ordinance", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "" }),
            block("item", { label: "Witnessed by", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Talk on the Holy Ghost", text: "" }),
            block("heading", { text: "Confirmation", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "" }),
            block("item", { label: "Primary Welcome", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Closing Hymn", text: "I Am a Child of God" }),
            block("item", { label: "Closing Prayer", text: "" }),
            block("item", { label: "Refreshments", text: "Please join us afterward" })
          ]
        },
        {
          id: "back",
          blocks: [
            block("spacer", { size: "medium" }),
            block("decoration", { style: "olive", size: "large" }),
            block("heading", { text: "Thank You for Joining Us", size: "medium", align: "center" }),
            block("text", { text: "We are grateful for your love and support on this special day.", style: "italic", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("text", { text: "Willing to bear one another's burdens, to mourn with those that mourn, and to stand as witnesses of God at all times.", style: "italic", align: "center" }),
            block("text", { text: "— Mosiah 18:8–9", style: "italic", align: "center" })
          ]
        }
      ]
    },
    "child-later": {
      label: "Child baptism only (confirmed later)",
      build: () => [
        {
          id: "front",
          blocks: [
            block("text", { text: "BAPTISM OF", style: "eyebrow", align: "center" }),
            block("heading", { text: "Name Placeholder", size: "large", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("image", { art: "baptism-river", size: "medium", shape: "square" }),
            block("text", { text: "August 30, 2026 · 10:00 AM", align: "center" }),
            block("text", { text: "Ward or Stake Name\nCity, State", style: "italic", align: "center" }),
            block("spacer", { size: "small" }),
            block("text", { text: "A covenant to follow Jesus Christ", style: "italic", align: "center" })
          ]
        },
        {
          id: "inside-left",
          blocks: [
            block("heading", { text: "Order of Service", size: "medium", align: "center" }),
            block("decoration", { style: "line", size: "medium" }),
            block("item", { label: "Presiding", text: "" }),
            block("item", { label: "Conducting", text: "" }),
            block("item", { label: "Pianist", text: "" }),
            block("item", { label: "Chorister", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Welcome", text: "" }),
            block("item", { label: "Opening Hymn", text: "When I Am Baptized" }),
            block("item", { label: "Opening Prayer", text: "" }),
            block("item", { label: "Talk on Baptism", text: "" })
          ]
        },
        {
          id: "inside-right",
          blocks: [
            block("heading", { text: "Baptismal Ordinance", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "" }),
            block("item", { label: "Witnessed by", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Testimonies", text: "" }),
            block("item", { label: "Closing Hymn", text: "I Am a Child of God" }),
            block("item", { label: "Closing Prayer", text: "" }),
            block("item", { label: "Refreshments", text: "Please join us afterward" }),
            block("text", { text: "Confirmation will take place during sacrament meeting.", style: "italic", align: "center" })
          ]
        },
        {
          id: "back",
          blocks: [
            block("spacer", { size: "medium" }),
            block("decoration", { style: "olive", size: "large" }),
            block("heading", { text: "Thank You for Joining Us", size: "medium", align: "center" }),
            block("text", { text: "We are grateful for your love and support on this special day.", style: "italic", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("text", { text: "And Jesus, when he was baptized, went up straightway out of the water: and, lo, the heavens were opened unto him, and he saw the Spirit of God descending like a dove, and lighting upon him.", style: "italic", align: "center" }),
            block("text", { text: "— Matthew 3:16", style: "italic", align: "center" })
          ]
        }
      ]
    },
    convert: {
      label: "Convert baptism",
      build: () => [
        {
          id: "front",
          blocks: [
            block("text", { text: "BAPTISM OF", style: "eyebrow", align: "center" }),
            block("heading", { text: "Name Placeholder", size: "large", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("image", { art: "christ-and-john", size: "medium", shape: "square" }),
            block("text", { text: "August 30, 2026 · 10:00 AM", align: "center" }),
            block("text", { text: "Ward or Stake Name\nCity, State", style: "italic", align: "center" }),
            block("spacer", { size: "small" }),
            block("text", { text: "A new beginning in the gospel of Jesus Christ", style: "italic", align: "center" })
          ]
        },
        {
          id: "inside-left",
          blocks: [
            block("heading", { text: "Order of Service", size: "medium", align: "center" }),
            block("decoration", { style: "line", size: "medium" }),
            block("item", { label: "Presiding", text: "" }),
            block("item", { label: "Conducting", text: "" }),
            block("item", { label: "Pianist", text: "" }),
            block("item", { label: "Chorister", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Welcome", text: "Ward Mission Leader" }),
            block("item", { label: "Opening Hymn", text: "" }),
            block("item", { label: "Invocation", text: "Missionary" }),
            block("item", { label: "Convert's Testimony", text: "" }),
            block("item", { label: "Talk on Baptism", text: "Friend" })
          ]
        },
        {
          id: "inside-right",
          blocks: [
            block("heading", { text: "Baptismal Ordinance", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "Missionary" }),
            block("item", { label: "Witnessed by", text: "Missionary, Friend" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Talk on the Holy Ghost", text: "Missionary" }),
            block("heading", { text: "Confirmation", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Bishopric Welcome", text: "Bishop" }),
            block("item", { label: "Closing Hymn", text: "" }),
            block("item", { label: "Benediction", text: "Missionary" }),
            block("item", { label: "Refreshments", text: "Please join us afterward" })
          ]
        },
        {
          id: "back",
          blocks: [
            block("spacer", { size: "medium" }),
            block("decoration", { style: "olive", size: "large" }),
            block("heading", { text: "Thank You for Joining Us", size: "medium", align: "center" }),
            block("item", { label: "Taught by", text: "Elder/Sister Name, Elder/Sister Name" }),
            block("text", { text: "We are grateful for your love and support on this special day.", style: "italic", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("text", { text: "Now faith is the assurance of things hoped for, the evidence of things not seen.", style: "italic", align: "center" }),
            block("text", { text: "— Hebrews 11:1", style: "italic", align: "center" })
          ]
        }
      ]
    },
    multiple: {
      label: "Multiple candidates",
      build: () => [
        {
          id: "front",
          blocks: [
            block("text", { text: "BAPTISM PROGRAM", style: "eyebrow", align: "center" }),
            block("heading", { text: "Baptism Program", size: "large", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("image", { art: "baptism-of-christ", size: "medium", shape: "square" }),
            block("text", { text: "August 30, 2026 · 10:00 AM", align: "center" }),
            block("text", { text: "Ward or Stake Name\nCity, State", style: "italic", align: "center" }),
            block("item", { label: "Being Baptized", text: "Name One, Name Two, Name Three" }),
            block("text", { text: "A covenant to follow Jesus Christ", style: "italic", align: "center" })
          ]
        },
        {
          id: "inside-left",
          blocks: [
            block("heading", { text: "Order of Service", size: "medium", align: "center" }),
            block("decoration", { style: "line", size: "medium" }),
            block("item", { label: "Presiding", text: "" }),
            block("item", { label: "Conducting", text: "" }),
            block("item", { label: "Pianist", text: "" }),
            block("item", { label: "Chorister", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Welcome", text: "" }),
            block("item", { label: "Opening Hymn", text: "When I Am Baptized" }),
            block("item", { label: "Opening Prayer", text: "" }),
            block("item", { label: "Talk on Baptism", text: "" })
          ]
        },
        {
          id: "inside-right",
          blocks: [
            block("heading", { text: "Baptismal Ordinances", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "" }),
            block("item", { label: "Witnessed by", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Talk on the Holy Ghost", text: "" }),
            block("heading", { text: "Confirmations", size: "small", align: "center" }),
            block("item", { label: "Performed by", text: "" }),
            block("decoration", { style: "line", size: "small" }),
            block("item", { label: "Closing Remarks", text: "" }),
            block("item", { label: "Closing Hymn", text: "I Am a Child of God" }),
            block("item", { label: "Closing Prayer", text: "" }),
            block("item", { label: "Refreshments", text: "Please join us afterward" })
          ]
        },
        {
          id: "back",
          blocks: [
            block("spacer", { size: "medium" }),
            block("decoration", { style: "olive", size: "large" }),
            block("heading", { text: "Thank You for Joining Us", size: "medium", align: "center" }),
            block("text", { text: "We are grateful for your love and support as we celebrate these baptisms.", style: "italic", align: "center" }),
            block("decoration", { style: "line", size: "small" }),
            block("text", { text: "Witnessing before the church that they have truly repented of all their sins, and are willing to take upon them the name of Jesus Christ, having a determination to serve him to the end.", style: "italic", align: "center" }),
            block("text", { text: "— Doctrine and Covenants 20:37", style: "italic", align: "center" })
          ]
        }
      ]
    },
    "service-one-hymn": {
      label: "One-page service + one hymn",
      build: () => onePageServiceTemplate("one-left")
    },
    "service-two-hymns": {
      label: "One-page service + two hymns inside",
      build: () => onePageServiceTemplate("two-left")
    },
    "service-split-hymns": {
      label: "One-page service + hymns split",
      build: () => onePageServiceTemplate("split")
    }
  };

  function defaultDocument() {
    return {
      theme: { ...themes.classic },
      pages: contentTemplates["service-one-hymn"].build()
    };
  }

  let documentState = restoreDraft() || defaultDocument();

  function currentPage() {
    return documentState.pages.find((page) => page.id === selectedPage);
  }

  function pageClipboardPayload(page) {
    return {
      format: "baptism-program-page",
      version: 1,
      copiedFrom: page.id,
      blocks: JSON.parse(JSON.stringify(page.blocks))
    };
  }

  function parsePageClipboard(value) {
    try {
      const candidate = typeof value === "string" ? JSON.parse(value) : value;
      if (candidate?.format !== "baptism-program-page" || candidate.version !== 1 || !Array.isArray(candidate.blocks)) return null;
      return candidate;
    } catch {
      return null;
    }
  }

  function openPageClipboardDatabase() {
    return new Promise((resolve, reject) => {
      if (!globalThis.indexedDB) {
        reject(new Error("Shared browser storage is unavailable."));
        return;
      }
      const request = indexedDB.open(pageClipboardDatabase, 1);
      request.onupgradeneeded = () => {
        if (!request.result.objectStoreNames.contains("pages")) request.result.createObjectStore("pages");
      };
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error || new Error("Could not open shared browser storage."));
      request.onblocked = () => reject(new Error("Shared browser storage is blocked."));
    });
  }

  async function storeSharedPageClipboard(payload) {
    const database = await openPageClipboardDatabase();
    await new Promise((resolve, reject) => {
      const transaction = database.transaction("pages", "readwrite");
      transaction.objectStore("pages").put(payload, "current-page");
      transaction.oncomplete = resolve;
      transaction.onerror = () => reject(transaction.error || new Error("Could not store the copied page."));
      transaction.onabort = () => reject(transaction.error || new Error("Could not store the copied page."));
    });
    database.close();
  }

  async function readSharedPageClipboard() {
    const database = await openPageClipboardDatabase();
    const value = await new Promise((resolve, reject) => {
      const request = database.transaction("pages", "readonly").objectStore("pages").get("current-page");
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error || new Error("Could not read the copied page."));
    });
    database.close();
    return parsePageClipboard(value);
  }

  async function copyCurrentPage() {
    pageClipboard = pageClipboardPayload(currentPage());
    const serialized = JSON.stringify(pageClipboard);
    try {
      localStorage.setItem(pageClipboardStorageKey, serialized);
    } catch {
      // IndexedDB supports larger image-rich pages when local storage is full.
    }
    try {
      sessionStorage.setItem(pageClipboardStorageKey, serialized);
    } catch {
      // The in-memory copy remains available in this window.
    }
    await Promise.allSettled([
      storeSharedPageClipboard(pageClipboard),
      navigator.clipboard?.writeText(serialized)
    ]);
    const label = pageDefinitions.find((entry) => entry.id === selectedPage)?.label || "Page";
    showStatus(`${label} copied. You can paste it into another program window.`);
  }

  async function readPageClipboard() {
    try {
      const fromSystem = parsePageClipboard(await navigator.clipboard?.readText());
      if (fromSystem) return fromSystem;
    } catch {
      // Fall through to shared browser storage.
    }
    if (pageClipboard) return pageClipboard;
    try {
      const shared = await readSharedPageClipboard();
      if (shared) return shared;
    } catch {
      // Fall through to the smaller browser-storage copies.
    }
    try {
      const local = parsePageClipboard(localStorage.getItem(pageClipboardStorageKey));
      if (local) return local;
      return parsePageClipboard(sessionStorage.getItem(pageClipboardStorageKey));
    } catch {
      return null;
    }
  }

  async function pasteCurrentPage() {
    const copied = await readPageClipboard();
    if (!copied) {
      showStatus("Copy a program page first, then paste it here.", true);
      return;
    }
    const destination = currentPage();
    const label = pageDefinitions.find((entry) => entry.id === selectedPage)?.label || "selected page";
    if (destination.blocks.length && !confirm(`Replace every element on ${label} with the copied page?`)) return;
    destination.blocks = copied.blocks.slice(0, 100).map(sanitizeBlock).filter(Boolean).map((item) => ({ ...item, id: id() }));
    renderBlockEditor();
    renderPages();
    scheduleSave();
    showStatus(`Copied page pasted onto ${label}.`);
  }

  function showStatus(message, isError = false) {
    clearTimeout(statusTimer);
    elements.status.textContent = message;
    elements.status.classList.toggle("error", isError);
    statusTimer = setTimeout(() => {
      elements.status.textContent = "";
      elements.status.classList.remove("error");
    }, 5000);
  }

  function storeDraft() {
    try {
      sessionStorage.setItem(draftKey, JSON.stringify(documentState));
    } catch {
      showStatus("This design is too large for browser draft storage. Download a save to keep it.", true);
    }
  }

  function scheduleSave() {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(storeDraft, 250);
  }

  function restoreDraft() {
    try {
      const saved = sessionStorage.getItem(draftKey);
      return saved ? sanitizeDocument(JSON.parse(saved)) : null;
    } catch {
      sessionStorage.removeItem(draftKey);
      return null;
    }
  }

  function sanitizeText(value, fallback = "") {
    return typeof value === "string" ? value.slice(0, 10000) : fallback;
  }

  function sanitizeChoice(value, choices, fallback) {
    return choices.includes(value) ? value : fallback;
  }

  function sanitizeTypography(candidate) {
    const value = candidate && typeof candidate === "object" ? candidate : {};
    return {
      weight: sanitizeChoice(value.weight, ["default", "bold", "regular"], "default"),
      slant: sanitizeChoice(value.slant, ["default", "italic", "regular"], "default"),
      color: /^#[0-9a-f]{6}$/i.test(value.color || "") ? value.color : "",
      font: sanitizeChoice(value.font, ["default", "serif", "sans", "soft"], "default")
    };
  }

  function sanitizeBlock(candidate) {
    if (!candidate || !blockNames[candidate.type]) return null;
    const clean = block(candidate.type);
    clean.id = sanitizeText(candidate.id, id()).slice(0, 100);
    if (typographyBlockTypes.includes(candidate.type)) clean.typography = sanitizeTypography(candidate.typography);
    if (candidate.type === "heading") {
      clean.text = sanitizeText(candidate.text, clean.text);
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "center");
      clean.color = sanitizeChoice(candidate.color, ["ink", "accent"], "ink");
    } else if (candidate.type === "text") {
      clean.text = sanitizeText(candidate.text, clean.text);
      clean.style = sanitizeChoice(candidate.style, ["normal", "italic", "eyebrow"], "normal");
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "center");
    } else if (candidate.type === "item") {
      clean.label = sanitizeText(candidate.label, clean.label);
      clean.text = sanitizeText(candidate.text, clean.text);
      clean.size = sanitizeChoice(candidate.size, ["compact", "cozy", "roomy"], "cozy");
      clean.style = sanitizeChoice(candidate.style, ["underline", "dotted", "plain", "none"], "underline");
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "left");
    } else if (candidate.type === "callout") {
      clean.title = sanitizeText(candidate.title, clean.title);
      clean.subtitle = sanitizeText(candidate.subtitle, clean.subtitle);
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "center");
    } else if (candidate.type === "hymn") {
      clean.title = sanitizeText(candidate.title, clean.title);
      clean.lyrics = sanitizeText(candidate.lyrics, clean.lyrics);
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "center");
      clean.lyricsAlign = sanitizeChoice(candidate.lyricsAlign, ["left", "center", "right"], "left");
      clean.columns = sanitizeChoice(candidate.columns, ["1", "2"], "1");
    } else if (candidate.type === "quote") {
      clean.text = sanitizeText(candidate.text, clean.text);
      clean.citation = sanitizeText(candidate.citation, clean.citation);
      clean.kind = sanitizeChoice(candidate.kind, ["scripture", "quote"], "scripture");
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "center");
    } else if (candidate.type === "markdown") {
      clean.text = sanitizeText(candidate.text, clean.text);
      clean.align = sanitizeChoice(candidate.align, ["left", "center", "right"], "left");
    } else if (candidate.type === "image") {
      clean.art = sanitizeChoice(candidate.art, builtInArtIds, "");
      clean.data = clean.art ? "" : (imagePattern.test(candidate.data || "") ? candidate.data : "");
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
      clean.shape = sanitizeChoice(candidate.shape, ["square", "soft", "circle"], "soft");
      clean.caption = sanitizeText(candidate.caption);
    } else if (candidate.type === "decoration") {
      const legacyStyles = { floral: "olive", waves: "water", dots: "rays", diamond: "temple" };
      clean.style = sanitizeChoice(legacyStyles[candidate.style] || candidate.style, decorationStyles, "olive");
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
      clean.data = imagePattern.test(candidate.data || "") ? candidate.data : "";
    } else if (candidate.type === "spacer") {
      clean.size = sanitizeChoice(candidate.size, ["small", "medium", "large"], "medium");
    }
    return clean;
  }

  function sanitizeDocument(candidate) {
    if (!candidate || typeof candidate !== "object") return null;
    const theme = candidate.theme || {};
    const color = (value, fallback) => /^#[0-9a-f]{6}$/i.test(value || "") ? value : fallback;
    const pages = pageDefinitions.map((definition) => {
      const source = Array.isArray(candidate.pages) ? candidate.pages.find((page) => page?.id === definition.id) : null;
      const blocks = Array.isArray(source?.blocks)
        ? source.blocks.slice(0, 100).map(sanitizeBlock).filter(Boolean)
        : [];
      return { id: definition.id, blocks };
    });
    return {
      theme: {
        paper: color(theme.paper, themes.classic.paper),
        text: color(theme.text, themes.classic.text),
        accent: color(theme.accent, themes.classic.accent),
        font: sanitizeChoice(theme.font, ["serif", "sans", "soft"], "serif"),
        monochrome: theme.monochrome === true
      },
      pages
    };
  }

  function legacyDocument(fields) {
    const old = fields || {};
    const migrated = defaultDocument();
    migrated.pages[0].blocks = [
      block("decoration", { style: "olive", size: "medium" }),
      block("text", { text: "BAPTISM PROGRAM", style: "eyebrow", align: "center" }),
      block("heading", { text: sanitizeText(old.name, "Name Placeholder"), size: "large", align: "center" }),
      block("decoration", { style: "line", size: "medium" }),
      block("text", { text: [old.date, old.time].filter(Boolean).join(" · "), align: "center" }),
      block("text", { text: [old.ward, old.location].filter(Boolean).join("\n"), style: "italic", align: "center" })
    ];
    if (imagePattern.test(old.front_image || "")) migrated.pages[0].blocks.push(block("image", { data: old.front_image, size: "large" }));

    const leftItems = [["Welcome", old.welcome], ["Opening hymn", old.opening_hymn], ["Opening prayer", old.opening_prayer], ["Talk on baptism", old.talk_baptism], ["Baptism ordinance", old.baptized_by]];
    migrated.pages[1].blocks = [block("heading", { text: "Order of Service" }), block("decoration")];
    leftItems.forEach(([label, text]) => migrated.pages[1].blocks.push(block("item", { label, text: sanitizeText(text) })));
    if (imagePattern.test(old.inside_left_image || "")) migrated.pages[1].blocks.push(block("image", { data: old.inside_left_image }));

    const rightItems = [["Talk on the Holy Ghost", old.talk_holy_ghost], ["Confirmation", old.confirmed_by], ["Testimonies", old.testimonies], ["Closing hymn", old.closing_hymn], ["Closing prayer", old.closing_prayer], ["Refreshments", old.refreshments]];
    migrated.pages[2].blocks = [block("heading", { text: "Order of Service" }), block("decoration")];
    rightItems.forEach(([label, text]) => migrated.pages[2].blocks.push(block("item", { label, text: sanitizeText(text) })));
    if (old.scripture_text) migrated.pages[2].blocks.push(block("text", { text: `${old.scripture_text}${old.scripture_reference ? `\n— ${old.scripture_reference}` : ""}`, style: "italic" }));
    if (imagePattern.test(old.inside_right_image || "")) migrated.pages[2].blocks.push(block("image", { data: old.inside_right_image }));

    migrated.pages[3].blocks = [];
    if (old.hymn_one_title) migrated.pages[3].blocks.push(block("heading", { text: old.hymn_one_title, size: "small" }));
    if (old.hymn_one_lyrics) migrated.pages[3].blocks.push(block("text", { text: old.hymn_one_lyrics, align: "left" }));
    if (old.hymn_two_title) migrated.pages[3].blocks.push(block("heading", { text: old.hymn_two_title, size: "small" }));
    if (old.hymn_two_lyrics) migrated.pages[3].blocks.push(block("text", { text: old.hymn_two_lyrics, align: "left" }));
    if (imagePattern.test(old.back_image || "")) migrated.pages[3].blocks.push(block("image", { data: old.back_image }));
    return migrated;
  }

  function make(tag, className, text) {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  }

  function decorationSvg(style) {
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.classList.add("decoration-symbol");
    svg.setAttribute("viewBox", "0 0 120 64");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("focusable", "false");

    const shape = (tag, attributes) => {
      const node = document.createElementNS("http://www.w3.org/2000/svg", tag);
      Object.entries(attributes).forEach(([name, value]) => node.setAttribute(name, value));
      svg.append(node);
    };

    if (style === "water") {
      shape("path", { d: "M60 7C52 19 48 25 48 32a12 12 0 0 0 24 0c0-7-4-13-12-25z", fill: "none" });
      shape("path", { d: "M31 43c8-5 18-7 29-7s21 2 29 7M19 51c11-6 25-9 41-9s30 3 41 9M9 58c14-7 31-10 51-10s37 3 51 10", fill: "none" });
    } else if (style === "jordan") {
      shape("path", { d: "M13 58c12-7 20-14 25-23 5-8 8-17 9-27M107 58c-12-7-20-14-25-23-5-8-8-17-9-27", fill: "none" });
      shape("path", { d: "M47 8c5 5 8 10 8 16 0 8-5 14-13 20-5 4-9 9-12 14M73 8c-5 5-8 10-8 16 0 8 5 14 13 20 5 4 9 9 12 14", fill: "none" });
      shape("path", { d: "M49 52c7-3 15-3 22 0M52 43c5-2 11-2 16 0M55 34c3-1 7-1 10 0", fill: "none" });
    } else if (style === "dove") {
      shape("path", { d: "M12 40c19 2 34-2 46-13 9-8 22-10 49-5-14 5-23 12-29 22-7 11-19 17-34 14-11-2-20-8-27-14z", fill: "none" });
      shape("path", { d: "M54 30C42 24 35 15 35 5c10 4 19 11 25 20", fill: "none" });
      shape("path", { d: "M27 50 14 58m22-4-8 8", fill: "none" });
      shape("circle", { cx: "88", cy: "27", r: "1.6", fill: "currentColor", stroke: "none" });
    } else if (style === "scriptures") {
      shape("path", { d: "M10 17c20-5 36-1 50 9v30c-14-10-30-14-50-9zm100 0c-20-5-36-1-50 9v30c14-10 30-14 50-9z", fill: "none" });
      shape("path", { d: "M60 26v30M18 25c13-2 24 0 34 5M18 34c13-2 24 0 34 5m50-14c-13-2-24 0-34 5m34 4c-13-2-24 0-34 5", fill: "none" });
    } else if (style === "temple") {
      shape("path", { d: "M8 58h104M16 58V45h19V34h13V22h8V11h8v11h8v12h13v11h19v13", fill: "none" });
      shape("path", { d: "M56 11 60 3l4 8M51 58V43h18v15M57 43V31h6v12", fill: "none" });
      shape("path", { d: "M23 50h5m8-9h6m36 0h6m8 9h5", fill: "none" });
    } else if (style === "tree-life") {
      shape("path", { d: "M60 58V34m0 8L45 27m15 9 16-14M52 35 36 38m32-8 17 4M60 34 58 17", fill: "none" });
      [[58, 11, 7], [42, 18, 8], [76, 16, 8], [29, 29, 7], [91, 29, 7], [47, 30, 8], [72, 29, 8], [38, 42, 6], [84, 41, 6]].forEach(([cx, cy, r]) => shape("circle", { cx, cy, r, fill: "none" }));
      shape("path", { d: "M45 59c5-5 10-7 15-7s10 2 15 7", fill: "none" });
    } else if (style === "rays") {
      shape("path", { d: "M60 5v16M25 17l12 12M8 47h18m69 0h17M95 17 83 29", fill: "none" });
      shape("path", { d: "M37 55c3-11 11-18 23-18s20 7 23 18", fill: "none" });
    }
    return svg;
  }

  function renderDecoration(target, decoration, forPrint) {
    target.className = `program-decoration decoration-${decoration.style} size-${decoration.size}`;
    target.setAttribute("role", "img");
    target.setAttribute("aria-label", decorationOptions.find(([value]) => value === decoration.style)?.[1] || "Decoration");
    if (decoration.style === "olive") target.append(make("span", "decoration-art"));
    else if (decoration.style === "custom") {
      if (imagePattern.test(decoration.data)) {
        const image = make("img", "decoration-custom-image");
        image.src = decoration.data;
        image.alt = "";
        target.append(image);
      } else if (!forPrint) {
        target.append(make("span", "decoration-placeholder", "Upload an image"));
      }
    } else if (decoration.style !== "line") target.append(decorationSvg(decoration.style));
  }

  function escapeHtml(value) {
    return value
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#39;");
  }

  function renderInlineMarkdown(line) {
    let html = escapeHtml(line);
    html = html.replace(/\*\*(.+?)\*\*|__(.+?)__/g, (match, a, b) => `<strong>${a ?? b}</strong>`);
    html = html.replace(/\*(.+?)\*|_(.+?)_/g, (match, a, b) => `<em>${a ?? b}</em>`);
    return html;
  }

  function renderMarkdown(source) {
    const text = typeof source === "string" ? source : "";
    return text
      .replace(/\r\n/g, "\n")
      .split(/\n{2,}/)
      .map((block) => {
        const trimmed = block.trim();
        if (!trimmed) return "";
        const lines = trimmed.split("\n").map((line) => line.trim());
        if (lines.every((line) => /^[-*]\s+/.test(line))) {
          return `<ul>${lines.map((line) => `<li>${renderInlineMarkdown(line.replace(/^[-*]\s+/, ""))}</li>`).join("")}</ul>`;
        }
        if (lines.every((line) => /^\d+[.)]\s+/.test(line))) {
          return `<ol>${lines.map((line) => `<li>${renderInlineMarkdown(line.replace(/^\d+[.)]\s+/, ""))}</li>`).join("")}</ol>`;
        }
        return `<p>${lines.map(renderInlineMarkdown).join("<br>")}</p>`;
      })
      .join("");
  }

  function applyTypography(node, typography) {
    if (!typography) return;
    if (typography.weight === "bold") node.style.fontWeight = "700";
    else if (typography.weight === "regular") node.style.fontWeight = "400";
    if (typography.slant === "italic") node.style.fontStyle = "italic";
    else if (typography.slant === "regular") node.style.fontStyle = "normal";
    if (typography.color) node.style.color = typography.color;
    if (typography.font && typography.font !== "default") node.style.fontFamily = fontStacks[typography.font];
  }

  function renderProgramBlock(item, forPrint) {
    if (item.type === "heading") {
      const node = make("h2", `program-heading size-${item.size} color-${item.color || "ink"}`, item.text);
      node.style.textAlign = item.align;
      applyTypography(node, item.typography);
      return node;
    }
    if (item.type === "text") {
      const node = make("p", `program-text style-${item.style}`, item.text);
      node.style.textAlign = item.align;
      applyTypography(node, item.typography);
      return node;
    }
    if (item.type === "item") {
      const node = make("div", `program-item size-${item.size || "cozy"} style-${item.style || "underline"} align-${item.align || "left"}`);
      const textSpan = make("span", "program-item-text", item.text);
      applyTypography(textSpan, item.typography);
      node.append(make("span", "program-item-label", item.label), textSpan);
      return node;
    }
    if (item.type === "callout") {
      const node = make("div", `program-callout size-${item.size} align-${item.align}`);
      const titleNode = make("h3", "program-callout-title", item.title);
      applyTypography(titleNode, item.typography);
      node.append(titleNode);
      if (item.subtitle) node.append(make("p", "program-callout-subtitle", item.subtitle));
      return node;
    }
    if (item.type === "hymn") {
      const node = make("div", `program-hymn size-${item.size} align-${item.align} columns-${item.columns || "1"}`);
      const lyricsNode = make("div", `program-hymn-lyrics lyrics-align-${item.lyricsAlign || "left"}`);
      applyTypography(lyricsNode, item.typography);
      (item.lyrics || "").replace(/\r\n/g, "\n").split(/\n{2,}/).forEach((verse) => {
        if (!verse.trim()) return;
        lyricsNode.append(make("p", "program-hymn-verse", verse));
      });
      node.append(make("h3", "program-hymn-title", item.title), lyricsNode);
      return node;
    }
    if (item.type === "quote") {
      const node = make("figure", `program-quote kind-${item.kind} size-${item.size} align-${item.align}`);
      const quoteNode = make("blockquote", "program-quote-text", item.text);
      applyTypography(quoteNode, item.typography);
      node.append(quoteNode);
      if (item.citation) node.append(make("figcaption", "program-quote-citation", item.citation));
      return node;
    }
    if (item.type === "markdown") {
      const node = make("div", "program-markdown");
      node.style.textAlign = item.align;
      node.innerHTML = renderMarkdown(item.text);
      applyTypography(node, item.typography);
      return node;
    }
    if (item.type === "image") {
      const artwork = builtInArt.find(({ id: artId }) => artId === item.art);
      const source = artwork?.src || (imagePattern.test(item.data) ? item.data : "");
      if (!source && forPrint) return make("span", "empty-print-block");
      const figure = make("figure", `program-image size-${item.size} shape-${item.shape}${artwork ? " built-in-art" : ""}`);
      if (source) {
        const image = make("img");
        image.alt = item.caption || artwork?.alt || "Program image";
        if (documentState.theme.monochrome) {
          const cached = grayscaleImageCache.get(source);
          image.src = typeof cached === "string" ? cached : source;
          getGrayscaleImageSrc(source).then((grayscaleSrc) => { image.src = grayscaleSrc; });
        } else {
          image.src = source;
        }
        figure.append(image);
      } else if (!forPrint) {
        figure.append(make("div", "image-placeholder", "Add an image"));
      }
      if (item.caption && source) figure.append(make("figcaption", "", item.caption));
      return figure;
    }
    if (item.type === "decoration") {
      if (item.style === "custom" && !imagePattern.test(item.data) && forPrint) return make("span", "empty-print-block");
      const node = make("div");
      renderDecoration(node, item, forPrint);
      return node;
    }
    return make("div", `program-spacer size-${item.size}`);
  }

  function toGrayscale(hex) {
    const match = /^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$/i.exec(hex || "");
    if (!match) return hex;
    const [r, g, b] = match.slice(1).map((part) => parseInt(part, 16));
    const gray = Math.round(0.299 * r + 0.587 * g + 0.114 * b).toString(16).padStart(2, "0");
    return `#${gray}${gray}${gray}`;
  }

  const grayscaleImageCache = new Map();

  function getGrayscaleImageSrc(src) {
    if (!grayscaleImageCache.has(src)) {
      const promise = new Promise((resolve) => {
        const loader = new Image();
        loader.onload = () => {
          try {
            const canvas = document.createElement("canvas");
            canvas.width = loader.naturalWidth || 1;
            canvas.height = loader.naturalHeight || 1;
            const ctx = canvas.getContext("2d");
            ctx.filter = "grayscale(1) contrast(1.12)";
            ctx.drawImage(loader, 0, 0);
            const dataUrl = canvas.toDataURL("image/png");
            grayscaleImageCache.set(src, dataUrl);
            resolve(dataUrl);
          } catch {
            grayscaleImageCache.set(src, src);
            resolve(src);
          }
        };
        loader.onerror = () => {
          grayscaleImageCache.set(src, src);
          resolve(src);
        };
        loader.src = src;
      });
      grayscaleImageCache.set(src, promise);
    }
    return Promise.resolve(grayscaleImageCache.get(src));
  }

  function buildProgramPage(page, forPrint = false) {
    const { monochrome } = documentState.theme;
    const outer = make("article", `program-page font-${documentState.theme.font}${monochrome ? " is-monochrome" : ""}`);
    outer.style.setProperty("--page-paper", monochrome ? toGrayscale(documentState.theme.paper) : documentState.theme.paper);
    outer.style.setProperty("--page-text", monochrome ? toGrayscale(documentState.theme.text) : documentState.theme.text);
    outer.style.setProperty("--page-accent", monochrome ? toGrayscale(documentState.theme.accent) : documentState.theme.accent);
    const inner = make("div", "program-page-inner");
    page.blocks.forEach((item, index) => {
      const rendered = renderProgramBlock(item, forPrint);
      rendered.classList.add("program-block");
      rendered.dataset.blockId = item.id;
      rendered.dataset.blockNumber = String(index + 1);
      inner.append(rendered);
    });
    outer.append(inner);
    return outer;
  }

  function renderTabs() {
    elements.pageTabs.replaceChildren();
    pageDefinitions.forEach((page, index) => {
      const wrapper = make("div", "page-tab");
      wrapper.dataset.selected = String(page.id === selectedPage);

      const select = make("button", "page-tab-select");
      select.type = "button";
      select.role = "tab";
      select.dataset.page = page.id;
      select.setAttribute("aria-selected", String(page.id === selectedPage));
      select.append(make("span", "page-number", String(index + 1)), make("span", "", page.label));
      select.addEventListener("click", () => selectPage(page.id));

      const move = make("div", "page-tab-move");
      const moveButton = (label, title, disabled, direction) => {
        const button = make("button", "icon-button", label);
        button.type = "button";
        button.title = title;
        button.setAttribute("aria-label", title);
        button.disabled = disabled;
        button.addEventListener("click", () => movePage(index, direction));
        return button;
      };
      move.append(
        moveButton("◀", "Swap with previous page", index === 0, -1),
        moveButton("▶", "Swap with next page", index === pageDefinitions.length - 1, 1)
      );

      wrapper.append(select, move);
      elements.pageTabs.append(wrapper);
    });
  }

  function movePage(index, direction) {
    const otherIndex = index + direction;
    if (otherIndex < 0 || otherIndex >= pageDefinitions.length) return;
    const pageA = documentState.pages.find((page) => page.id === pageDefinitions[index].id);
    const pageB = documentState.pages.find((page) => page.id === pageDefinitions[otherIndex].id);
    [pageA.blocks, pageB.blocks] = [pageB.blocks, pageA.blocks];
    renderBlockEditor();
    renderPages();
    scheduleSave();
  }

  function renderPages() {
    elements.livePreview.replaceChildren();
    documentState.pages.forEach((page) => {
      const definition = pageDefinitions.find((entry) => entry.id === page.id);
      const shell = make("button", `page-preview-shell${page.id === selectedPage ? " selected" : ""}`);
      shell.type = "button";
      shell.dataset.page = page.id;
      shell.setAttribute("aria-label", `Edit ${definition.label}`);
      const label = make("span", "preview-page-label", definition.label);
      const viewport = make("span", "page-preview-viewport");
      const scaled = make("span", "page-preview-scale");
      scaled.append(buildProgramPage(page));
      viewport.append(scaled);
      shell.append(label, viewport);
      shell.addEventListener("click", () => selectPage(page.id));
      elements.livePreview.append(shell);

      shell.querySelectorAll("[data-block-id]").forEach((previewBlock) => {
        const blockId = previewBlock.dataset.blockId;
        previewBlock.addEventListener("pointerenter", () => setLinkedHighlight(blockId));
        previewBlock.addEventListener("pointerleave", () => setLinkedHighlight(""));
        previewBlock.addEventListener("click", (event) => {
          event.stopPropagation();
          if (page.id !== selectedPage) selectPage(page.id);
          requestAnimationFrame(() => {
            const card = [...elements.blockList.querySelectorAll(".block-card")].find((entry) => entry.dataset.blockId === blockId);
            card?.scrollIntoView({ behavior: "smooth", block: "center" });
            setLinkedHighlight(blockId);
          });
        });
      });

      document.querySelectorAll(`[data-print-page="${page.id}"]`).forEach((target) => {
        target.replaceChildren(buildProgramPage(page, true));
      });
    });

    requestAnimationFrame(() => {
      elements.livePreview.querySelectorAll(".page-preview-shell").forEach((shell) => {
        const inner = shell.querySelector(".program-page-inner");
        const overflowing = inner.scrollHeight > inner.clientHeight + 1;
        shell.classList.toggle("has-overflow", overflowing);
        if (overflowing) shell.append(make("span", "overflow-warning", "Too much content"));
      });
      applyLinkedHighlight();
    });
  }

  function applyLinkedHighlight() {
    document.querySelectorAll(".block-card[data-block-id], .page-preview-shell [data-block-id]").forEach((node) => {
      node.classList.toggle("is-linked-highlight", Boolean(highlightedBlockId) && node.dataset.blockId === highlightedBlockId);
    });
  }

  function setLinkedHighlight(blockId) {
    highlightedBlockId = blockId;
    applyLinkedHighlight();
  }

  function selectPage(pageId) {
    selectedPage = pageId;
    renderTabs();
    renderBlockEditor();
    renderPages();
  }

  function updateFocusMode() {
    elements.livePreview.classList.toggle("is-focused", focusMode);
    elements.livePreview.style.setProperty("--focus-width", `${Math.round(528 * previewZoom)}px`);
    elements.livePreview.style.setProperty("--focus-height", `${Math.round(816 * previewZoom)}px`);
    elements.livePreview.style.setProperty("--focus-transform", `scale(${previewZoom})`);
    elements.toggleFocus.setAttribute("aria-pressed", String(focusMode));
    elements.toggleFocus.textContent = focusMode ? "Show all pages" : "Expand page";
    elements.zoomValue.value = `${Math.round(previewZoom * 100)}%`;
    elements.zoomValue.textContent = elements.zoomValue.value;
    elements.zoomOut.disabled = previewZoom <= .6;
    elements.zoomIn.disabled = previewZoom >= 1.2;
  }

  function labelControl(text, control) {
    const label = make("label", "field-control");
    label.append(make("span", "", text), control);
    return label;
  }

  function inputControl(value, onInput) {
    const input = document.createElement("input");
    input.value = value;
    input.addEventListener("input", () => onInput(input.value));
    return input;
  }

  function textareaControl(value, onInput, rows = 3) {
    const textarea = document.createElement("textarea");
    textarea.rows = rows;
    textarea.value = value;
    textarea.addEventListener("input", () => onInput(textarea.value));
    return textarea;
  }

  function selectControl(value, options, onChange) {
    const select = document.createElement("select");
    options.forEach(([optionValue, label]) => {
      const option = make("option", "", label);
      option.value = optionValue;
      option.selected = optionValue === value;
      select.append(option);
    });
    select.addEventListener("change", () => onChange(select.value));
    return select;
  }

  function changeBlock(item, property, value) {
    item[property] = value;
    renderPages();
    scheduleSave();
  }

  function changeTypography(item, property, value) {
    item.typography[property] = value;
    renderPages();
    scheduleSave();
  }

  function colorOverrideControl(value, onChange, onClear) {
    const wrap = make("div", "typography-color-control");
    const input = document.createElement("input");
    input.type = "color";
    input.value = value || "#000000";
    input.addEventListener("input", () => onChange(input.value));
    const clear = make("button", "quiet-button", "Use theme color");
    clear.type = "button";
    clear.addEventListener("click", () => onClear());
    wrap.append(input, clear);
    return wrap;
  }

  function renderTypographyControls(card, item, note) {
    const details = document.createElement("details");
    details.className = "advanced-style";
    const summary = make("summary", "", "More styling: bold, italic, color, font");
    details.append(summary);
    if (note) details.append(make("p", "advanced-style-note", note));
    details.append(makeRow([
      labelControl("Weight", selectControl(item.typography.weight, [["default", "Default"], ["bold", "Bold"], ["regular", "Regular"]], (value) => changeTypography(item, "weight", value))),
      labelControl("Slant", selectControl(item.typography.slant, [["default", "Default"], ["italic", "Italic"], ["regular", "Upright"]], (value) => changeTypography(item, "slant", value)))
    ]));
    details.append(labelControl("Font", selectControl(item.typography.font, [["default", "Match page font"], ["serif", "Classic serif"], ["sans", "Clean sans serif"], ["soft", "Soft rounded"]], (value) => changeTypography(item, "font", value))));
    details.append(labelControl("Color", colorOverrideControl(item.typography.color, (value) => changeTypography(item, "color", value), () => { details.querySelector('input[type="color"]').value = "#000000"; changeTypography(item, "color", ""); })));
    card.append(details);
  }

  function renderBlockFields(card, item) {
    if (item.type === "heading") {
      card.append(labelControl("Text", inputControl(item.text, (value) => changeBlock(item, "text", value))));
      card.append(makeRow([
        labelControl("Size", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value))),
        labelControl("Align", selectControl(item.align, [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value)))
      ]));
      card.append(labelControl("Color", selectControl(item.color || "ink", [["ink", "Ink (matches text)"], ["accent", "Accent color"]], (value) => changeBlock(item, "color", value))));
      renderTypographyControls(card, item);
    } else if (item.type === "text") {
      card.append(labelControl("Text", textareaControl(item.text, (value) => changeBlock(item, "text", value))));
      card.append(makeRow([
        labelControl("Style", selectControl(item.style, [["normal", "Normal"], ["italic", "Italic"], ["eyebrow", "Small caps"]], (value) => changeBlock(item, "style", value))),
        labelControl("Align", selectControl(item.align, [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value)))
      ]));
      renderTypographyControls(card, item);
    } else if (item.type === "item") {
      card.append(labelControl("Label", inputControl(item.label, (value) => changeBlock(item, "label", value))));
      card.append(labelControl("Name or details", inputControl(item.text, (value) => changeBlock(item, "text", value))));
      card.append(makeRow([
        labelControl("Line style", selectControl(item.style || "underline", [["underline", "Underline"], ["dotted", "Dotted leader"], ["plain", "Simple, no line"], ["none", "No line"]], (value) => changeBlock(item, "style", value))),
        labelControl("Spacing", selectControl(item.size || "cozy", [["compact", "Compact"], ["cozy", "Cozy"], ["roomy", "Roomy"]], (value) => changeBlock(item, "size", value)))
      ]));
      card.append(labelControl("Align", selectControl(item.align || "left", [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value))));
      renderTypographyControls(card, item, "Applies to the name or details text (not the label).");
    } else if (item.type === "callout") {
      card.append(labelControl("Title", inputControl(item.title, (value) => changeBlock(item, "title", value))));
      card.append(labelControl("Subtitle", inputControl(item.subtitle, (value) => changeBlock(item, "subtitle", value))));
      card.append(makeRow([
        labelControl("Size", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value))),
        labelControl("Align", selectControl(item.align, [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value)))
      ]));
      renderTypographyControls(card, item, "Applies to the title (not the subtitle).");
    } else if (item.type === "hymn") {
      card.append(labelControl("Title", inputControl(item.title, (value) => changeBlock(item, "title", value))));
      card.append(labelControl("Lyrics", textareaControl(item.lyrics, (value) => changeBlock(item, "lyrics", value), 10)));
      card.append(make("p", "field-hint", "Leave a blank line between verses so they can wrap or split into columns cleanly."));
      card.append(makeRow([
        labelControl("Size", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value))),
        labelControl("Title align", selectControl(item.align, [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value)))
      ]));
      card.append(labelControl("Lyrics align", selectControl(item.lyricsAlign || "left", [["left", "Left (recommended)"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "lyricsAlign", value))));
      card.append(make("p", "field-hint", "Center or right lyrics can look ragged once a line wraps onto a second line — left works best for most hymns."));
      card.append(labelControl("Layout", selectControl(item.columns || "1", [["1", "One column"], ["2", "Two columns (fits more per page)"]], (value) => changeBlock(item, "columns", value))));
      renderTypographyControls(card, item, "Applies to the lyrics (not the title).");
    } else if (item.type === "quote") {
      card.append(labelControl("Passage", textareaControl(item.text, (value) => changeBlock(item, "text", value), 5)));
      card.append(labelControl("Reference or attribution", inputControl(item.citation, (value) => changeBlock(item, "citation", value))));
      card.append(makeRow([
        labelControl("Type", selectControl(item.kind, [["scripture", "Scripture"], ["quote", "Quotation"]], (value) => changeBlock(item, "kind", value))),
        labelControl("Size", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value)))
      ]));
      card.append(labelControl("Align", selectControl(item.align, [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value))));
      renderTypographyControls(card, item, "Applies to the passage (not the reference).");
    } else if (item.type === "markdown") {
      card.append(labelControl("Text", textareaControl(item.text, (value) => changeBlock(item, "text", value), 6)));
      card.append(make("p", "field-hint", 'Supports **bold**, *italic*, and blank lines for new paragraphs. For a list, put every item on its own line starting with "- " (or "1. "), separated from other text by a blank line.'));
      card.append(labelControl("Align", selectControl(item.align, [["left", "Left"], ["center", "Center"], ["right", "Right"]], (value) => changeBlock(item, "align", value))));
      renderTypographyControls(card, item);
    } else if (item.type === "image") {
      const artwork = builtInArt.find(({ id: artId }) => artId === item.art);
      const source = artwork?.src || (imagePattern.test(item.data) ? item.data : "");
      if (source) {
        const thumbnail = make("img", "editor-thumbnail");
        thumbnail.src = source;
        thumbnail.alt = item.caption || artwork?.alt || "Selected image";
        thumbnail.classList.toggle("is-built-in", Boolean(artwork));
        card.append(thumbnail);
      }

      card.append(make("span", "art-library-title", "Built-in art"));
      [
        { id: "lds", label: "Latter-day Saint photography" },
        { id: "human", label: "Human-drawn Bible artwork" },
        { id: "ai", label: "AI-generated artwork" }
      ].forEach((group) => {
        card.append(make("span", "art-library-group-title", group.label));
        const library = make("div", "art-library");
        builtInArt.filter((art) => art.group === group.id).forEach((art) => {
          const choice = make("button", `art-choice${item.art === art.id ? " selected" : ""}`);
          choice.type = "button";
          choice.setAttribute("aria-pressed", String(item.art === art.id));
          choice.append(make("img"), make("span", "", art.name));
          choice.querySelector("img").src = art.src;
          choice.querySelector("img").alt = "";
          choice.addEventListener("click", () => {
            item.art = art.id;
            item.data = "";
            item.shape = "square";
            renderBlockEditor();
            renderPages();
            scheduleSave();
          });
          library.append(choice);
        });
        card.append(library);

        if (group.id === "lds") {
          const credit = make("p", "art-library-credit", "Independently sourced LDS imagery · ");
          const details = document.createElement("a");
          details.href = "static/art/ATTRIBUTION.md";
          details.target = "_blank";
          details.textContent = "licenses and sources";
          credit.append(details, document.createTextNode(" · not an official Church product"));
          card.append(credit);
        } else if (group.id === "human") {
          const credit = make("p", "art-library-credit", "Drawn by Jim Padgett · ");
          const license = document.createElement("a");
          license.href = "https://creativecommons.org/licenses/by-sa/3.0/";
          license.target = "_blank";
          license.rel = "noreferrer";
          license.textContent = "CC BY-SA 3.0";
          credit.append(license, document.createTextNode(" · "));
          const details = document.createElement("a");
          details.href = "static/art/ATTRIBUTION.md";
          details.target = "_blank";
          details.textContent = "sources";
          credit.append(details);
          card.append(credit);
        } else {
          card.append(make("p", "art-library-credit", "Created with AI for this project."));
        }
      });

      const actions = make("div", "image-actions");
      const picker = make("label", "image-picker", "Upload your own");
      const file = document.createElement("input");
      file.type = "file";
      file.accept = "image/jpeg,image/png,image/webp";
      file.hidden = true;
      file.addEventListener("change", () => handleImage(file, item));
      picker.append(file);
      actions.append(picker);
      if (source) {
        const clear = make("button", "quiet-button danger", "Clear image");
        clear.type = "button";
        clear.addEventListener("click", () => {
          item.data = "";
          item.art = "";
          renderBlockEditor();
          renderPages();
          storeDraft();
        });
        actions.append(clear);
      }
      card.append(actions);
      card.append(makeRow([
        labelControl("Size", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value))),
        labelControl("Shape", selectControl(item.shape, [["square", "Square"], ["soft", "Rounded"], ["circle", "Circle"]], (value) => changeBlock(item, "shape", value)))
      ]));
      card.append(labelControl("Caption", inputControl(item.caption, (value) => changeBlock(item, "caption", value))));
    } else if (item.type === "decoration") {
      card.append(makeRow([
        labelControl("Symbol", selectControl(item.style, decorationOptions, (value) => {
          item.style = value;
          renderBlockEditor();
          renderPages();
          scheduleSave();
        })),
        labelControl("Size", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value)))
      ]));
      if (item.style === "custom") {
        if (imagePattern.test(item.data)) {
          const thumbnail = make("img", "editor-thumbnail is-built-in");
          thumbnail.src = item.data;
          thumbnail.alt = "Custom decoration";
          card.append(thumbnail);
        }
        const actions = make("div", "image-actions");
        const picker = make("label", "image-picker", item.data ? "Replace image" : "Upload your own");
        const file = document.createElement("input");
        file.type = "file";
        file.accept = "image/jpeg,image/png,image/webp";
        file.hidden = true;
        file.addEventListener("change", () => handleDecorationImage(file, item));
        picker.append(file);
        actions.append(picker);
        if (item.data) {
          const clear = make("button", "quiet-button danger", "Clear image");
          clear.type = "button";
          clear.addEventListener("click", () => {
            item.data = "";
            renderBlockEditor();
            renderPages();
            storeDraft();
          });
          actions.append(clear);
        }
        card.append(actions);
        card.append(make("p", "field-hint", "A small PNG with a transparent background works best, like a simple line drawing or symbol."));
      }
    } else {
      card.append(labelControl("Amount", selectControl(item.size, [["small", "Small"], ["medium", "Medium"], ["large", "Large"]], (value) => changeBlock(item, "size", value))));
    }
  }

  function makeRow(children) {
    const row = make("div", "control-row");
    row.append(...children);
    return row;
  }

  function renderBlockEditor() {
    const page = currentPage();
    const definition = pageDefinitions.find((entry) => entry.id === page.id);
    elements.pageLabel.textContent = definition.label;
    elements.elementCount.textContent = `${page.blocks.length} element${page.blocks.length === 1 ? "" : "s"}`;
    elements.blockList.replaceChildren();

    if (page.blocks.length === 0) {
      elements.blockList.append(make("div", "empty-page", "This page is empty. Add an element below."));
    }

    page.blocks.forEach((item, index) => {
      const card = make("article", "block-card");
      card.dataset.blockId = item.id;
      const header = make("header", "block-card-header");
      const title = make("div", "block-card-title");
      title.append(make("span", "link-number", String(index + 1)), make("strong", "", blockNames[item.type]));
      header.append(title);
      const actions = make("div", "block-actions");
      const action = (label, title, disabled, handler, className = "") => {
        const button = make("button", `icon-button ${className}`, label);
        button.type = "button";
        button.title = title;
        button.setAttribute("aria-label", title);
        button.disabled = disabled;
        button.addEventListener("click", handler);
        return button;
      };
      actions.append(
        action("↑", "Move up", index === 0, () => moveBlock(index, -1)),
        action("↓", "Move down", index === page.blocks.length - 1, () => moveBlock(index, 1)),
        action("⧉", "Duplicate", false, () => duplicateBlock(index)),
        action("×", "Remove", false, () => removeBlock(index), "remove")
      );
      header.append(actions);
      card.append(header);
      renderBlockFields(card, item);
      card.addEventListener("pointerenter", () => setLinkedHighlight(item.id));
      card.addEventListener("pointerleave", () => {
        if (!card.contains(document.activeElement)) setLinkedHighlight("");
      });
      card.addEventListener("focusin", () => setLinkedHighlight(item.id));
      card.addEventListener("focusout", (event) => {
        if (!card.contains(event.relatedTarget)) setLinkedHighlight("");
      });
      elements.blockList.append(card);
    });
    applyLinkedHighlight();
  }

  function moveBlock(index, direction) {
    const blocks = currentPage().blocks;
    const [item] = blocks.splice(index, 1);
    blocks.splice(index + direction, 0, item);
    renderBlockEditor();
    renderPages();
    scheduleSave();
  }

  function duplicateBlock(index) {
    const blocks = currentPage().blocks;
    const copy = JSON.parse(JSON.stringify(blocks[index]));
    copy.id = id();
    blocks.splice(index + 1, 0, copy);
    renderBlockEditor();
    renderPages();
    scheduleSave();
  }

  function removeBlock(index) {
    currentPage().blocks.splice(index, 1);
    renderBlockEditor();
    renderPages();
    scheduleSave();
  }

  function resizeImage(file) {
    return new Promise((resolve, reject) => {
      const objectUrl = URL.createObjectURL(file);
      const source = new Image();
      source.onload = () => {
        const maxDimension = 1400;
        const scale = Math.min(1, maxDimension / Math.max(source.naturalWidth, source.naturalHeight));
        const canvas = document.createElement("canvas");
        canvas.width = Math.max(1, Math.round(source.naturalWidth * scale));
        canvas.height = Math.max(1, Math.round(source.naturalHeight * scale));
        const context = canvas.getContext("2d");
        context.fillStyle = "#fff";
        context.fillRect(0, 0, canvas.width, canvas.height);
        context.drawImage(source, 0, 0, canvas.width, canvas.height);
        URL.revokeObjectURL(objectUrl);
        resolve(canvas.toDataURL("image/jpeg", 0.84));
      };
      source.onerror = () => {
        URL.revokeObjectURL(objectUrl);
        reject(new Error("That image could not be read."));
      };
      source.src = objectUrl;
    });
  }

  async function handleImage(input, item) {
    const file = input.files[0];
    if (!file) return;
    if (!/image\/(?:jpeg|png|webp)/.test(file.type) || file.size > 12 * 1024 * 1024) {
      showStatus("Choose a JPEG, PNG, or WebP image smaller than 12 MB.", true);
      return;
    }
    try {
      showStatus("Preparing image…");
      item.data = await resizeImage(file);
      item.art = "";
      renderBlockEditor();
      renderPages();
      storeDraft();
      showStatus("Image added.");
    } catch (error) {
      showStatus(error.message, true);
    }
  }

  function resizeDecorationImage(file) {
    return new Promise((resolve, reject) => {
      const objectUrl = URL.createObjectURL(file);
      const source = new Image();
      source.onload = () => {
        const maxDimension = 600;
        const scale = Math.min(1, maxDimension / Math.max(source.naturalWidth, source.naturalHeight));
        const canvas = document.createElement("canvas");
        canvas.width = Math.max(1, Math.round(source.naturalWidth * scale));
        canvas.height = Math.max(1, Math.round(source.naturalHeight * scale));
        const context = canvas.getContext("2d");
        context.drawImage(source, 0, 0, canvas.width, canvas.height);
        URL.revokeObjectURL(objectUrl);
        resolve(canvas.toDataURL("image/png"));
      };
      source.onerror = () => {
        URL.revokeObjectURL(objectUrl);
        reject(new Error("That image could not be read."));
      };
      source.src = objectUrl;
    });
  }

  async function handleDecorationImage(input, item) {
    const file = input.files[0];
    if (!file) return;
    if (!/image\/(?:jpeg|png|webp)/.test(file.type) || file.size > 12 * 1024 * 1024) {
      showStatus("Choose a JPEG, PNG, or WebP image smaller than 12 MB.", true);
      return;
    }
    try {
      showStatus("Preparing image…");
      item.data = await resizeDecorationImage(file);
      renderBlockEditor();
      renderPages();
      storeDraft();
      showStatus("Decoration added.");
    } catch (error) {
      showStatus(error.message, true);
    }
  }

  function updateThemeInputs() {
    elements.paper.value = documentState.theme.paper;
    elements.text.value = documentState.theme.text;
    elements.accent.value = documentState.theme.accent;
    elements.font.value = documentState.theme.font;
    elements.monochrome.checked = documentState.theme.monochrome;
    const match = Object.entries(themes).find(([, theme]) => Object.keys(theme).every((key) => theme[key] === documentState.theme[key]));
    elements.preset.value = match?.[0] || "custom";
  }

  elements.addBlock.addEventListener("click", () => {
    currentPage().blocks.push(block(elements.newBlockType.value));
    renderBlockEditor();
    renderPages();
    scheduleSave();
    elements.blockList.lastElementChild?.scrollIntoView({ behavior: "smooth", block: "center" });
  });

  elements.contentTemplate?.addEventListener("change", () => {
    const key = elements.contentTemplate.value;
    const template = contentTemplates[key];
    elements.contentTemplate.value = "";
    if (!template) return;
    if (!confirm(`Replace the content on all four pages with the "${template.label}" template? Your current text and images will be lost.`)) return;
    documentState.pages = template.build();
    selectedPage = "front";
    renderTabs();
    renderBlockEditor();
    renderPages();
    scheduleSave();
    showStatus("Template applied.");
  });

  elements.preset.addEventListener("change", () => {
    if (elements.preset.value === "custom") return;
    documentState.theme = { ...themes[elements.preset.value] };
    updateThemeInputs();
    renderPages();
    scheduleSave();
  });

  [[elements.paper, "paper"], [elements.text, "text"], [elements.accent, "accent"], [elements.font, "font"]].forEach(([control, property]) => {
    control.addEventListener("input", () => {
      documentState.theme[property] = control.value;
      elements.preset.value = "custom";
      renderPages();
      scheduleSave();
    });
  });

  elements.monochrome.addEventListener("change", () => {
    documentState.theme.monochrome = elements.monochrome.checked;
    elements.preset.value = "custom";
    renderPages();
    scheduleSave();
  });

  document.querySelector("#reset-theme").addEventListener("click", () => {
    documentState.theme = { ...themes.classic };
    updateThemeInputs();
    renderPages();
    scheduleSave();
  });

  document.querySelector("#download-save").addEventListener("click", () => {
    const save = { format: "baptism-program", version: 2, savedAt: new Date().toISOString(), document: documentState };
    const blob = new Blob([JSON.stringify(save, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    const firstHeading = documentState.pages[0].blocks.find((item) => item.type === "heading")?.text || "baptism-program";
    const filename = firstHeading.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "baptism-program";
    link.href = url;
    link.download = `${filename}.baptism.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 0);
    showStatus("Save downloaded.");
  });

  elements.load.addEventListener("change", async () => {
    const file = elements.load.files[0];
    if (!file) return;
    try {
      if (file.size > 20 * 1024 * 1024) throw new Error("That save is larger than the 20 MB limit.");
      const save = JSON.parse(await file.text());
      let loaded;
      if (save.format === "baptism-program" && save.version === 2) loaded = sanitizeDocument(save.document);
      else if (save.format === "baptism-program" && save.version === 1) loaded = legacyDocument(save.fields);
      if (!loaded) throw new Error("This is not a supported baptism program save.");
      documentState = loaded;
      selectedPage = "front";
      updateThemeInputs();
      renderTabs();
      renderBlockEditor();
      renderPages();
      storeDraft();
      showStatus(save.version === 1 ? "Older save loaded and upgraded." : "Save loaded.");
    } catch (error) {
      showStatus(error instanceof SyntaxError ? "The selected file is not valid JSON." : error.message, true);
    } finally {
      elements.load.value = "";
    }
  });

  document.querySelector("#print-program").addEventListener("click", () => {
    document.body.classList.remove("print-mode-sample");
    document.querySelector("#sample-page-size-override")?.remove();
    window.print();
  });

  document.querySelector("#print-sample").addEventListener("click", () => {
    const pageSizeOverride = make("style", "", "@page { size: 5.5in 8.5in; margin: 0; }");
    pageSizeOverride.id = "sample-page-size-override";
    document.head.append(pageSizeOverride);
    document.body.classList.add("print-mode-sample");
    const cleanup = () => {
      document.body.classList.remove("print-mode-sample");
      pageSizeOverride.remove();
      window.removeEventListener("afterprint", cleanup);
    };
    window.addEventListener("afterprint", cleanup);
    window.print();
  });

  elements.toggleFocus.addEventListener("click", () => {
    focusMode = !focusMode;
    updateFocusMode();
  });

  elements.copyPage.addEventListener("click", copyCurrentPage);
  elements.pastePage.addEventListener("click", pasteCurrentPage);
  elements.zoomOut.addEventListener("click", () => {
    previewZoom = Math.max(.6, Math.round((previewZoom - .1) * 10) / 10);
    focusMode = true;
    updateFocusMode();
  });
  elements.zoomIn.addEventListener("click", () => {
    previewZoom = Math.min(1.2, Math.round((previewZoom + .1) * 10) / 10);
    focusMode = true;
    updateFocusMode();
  });

  updateThemeInputs();
  updateFocusMode();
  renderTabs();
  renderBlockEditor();
  renderPages();
})();
