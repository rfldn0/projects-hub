// Progressive enhancement: filters, unrolling panels and arrow keys.
// Every page is fully rendered HTML and works without this file.
(() => {
  "use strict";

  const doc = document;
  const wide = matchMedia("(min-width: 960px)");
  const calm = matchMedia("(prefers-reduced-motion: reduce)");
  const ignoreKey = (e) =>
    e.altKey || e.ctrlKey || e.metaKey || /^(INPUT|TEXTAREA|SELECT)$/.test(e.target.tagName);

  const track = doc.querySelector("[data-track]");

  if (!track) {
    // Project pages: arrow keys follow the previous / next links.
    doc.addEventListener("keydown", (e) => {
      if (ignoreKey(e)) return;
      const sel = { ArrowLeft: "[data-prev]", ArrowRight: "[data-next]" }[e.key];
      const link = sel && doc.querySelector(sel);
      if (link) location.href = link.href;
    });
    return;
  }

  const panels = [...track.querySelectorAll(".panel")];
  const dots = [...doc.querySelectorAll(".dot")];
  const chips = [...doc.querySelectorAll("[data-filter]")];
  let current = panels.find((p) => p.classList.contains("is-sel")) || panels[0];

  const shown = () => panels.filter((p) => !p.hidden);

  function reveal(panel) {
    // Wait out the width transition so we scroll to where the panel ends up.
    // On phones the unrolled panel is wider than the screen, so pin its start edge.
    setTimeout(() => {
      panel.scrollIntoView({
        behavior: calm.matches ? "auto" : "smooth",
        block: "nearest",
        inline: wide.matches ? "nearest" : "start",
      });
    }, calm.matches ? 0 : 520);
  }

  function select(panel, scroll) {
    if (!panel) return;
    current = panel;
    for (const p of panels) p.classList.toggle("is-sel", p === panel);
    for (const d of dots) d.setAttribute("aria-current", String(d.dataset.id === panel.dataset.id));
    if (scroll) reveal(panel);
  }

  function step(by) {
    const list = shown();
    if (!list.length) return;
    const i = list.indexOf(current);
    select(list[(i + by + list.length) % list.length], true);
  }

  // First click previews a panel; clicking the previewed panel opens its page.
  function open(panel) {
    if (panel === current && panel.classList.contains("is-sel")) location.href = panel.dataset.href;
    else select(panel, true);
  }

  function filter(status, remember) {
    if (!chips.some((c) => c.dataset.filter === status)) status = "all";
    const hide = (el) => status !== "all" && el.dataset.status !== status;
    for (const p of panels) p.hidden = hide(p);
    for (const d of dots) d.hidden = hide(d);
    for (const c of chips) c.setAttribute("aria-pressed", String(c.dataset.filter === status));
    select(shown()[0], false);
    track.scrollLeft = 0;
    if (remember) {
      try {
        history.replaceState(null, "", status === "all" ? location.pathname : "#" + status);
      } catch (_) {
        /* file:// in some browsers */
      }
    }
  }

  track.addEventListener("click", (e) => {
    const panel = e.target.closest(".panel");
    if (!panel) return;
    const link = e.target.closest("a");
    if (link && !link.classList.contains("panel-cta")) return; // Open project, Repo
    if (link && (e.ctrlKey || e.metaKey || e.shiftKey)) return; // new tab / window
    if (link) e.preventDefault();
    open(panel);
  });

  // Hold-and-drag with a mouse; touch and trackpads already scroll natively.
  let drag = null;
  let dragged = false;
  track.addEventListener("pointerdown", (e) => {
    if (e.pointerType !== "mouse" || e.button !== 0) return;
    drag = { id: e.pointerId, x: e.clientX, left: track.scrollLeft };
    dragged = false;
  });
  track.addEventListener("pointermove", (e) => {
    if (!drag || e.pointerId !== drag.id) return;
    const dx = e.clientX - drag.x;
    if (!dragged) {
      if (Math.abs(dx) < 6) return; // still a click
      dragged = true;
      track.setPointerCapture(drag.id);
      track.classList.add("is-dragging");
      getSelection()?.removeAllRanges();
    }
    track.scrollLeft = drag.left - dx;
  });
  const endDrag = () => {
    drag = null;
    track.classList.remove("is-dragging");
  };
  track.addEventListener("pointerup", endDrag);
  track.addEventListener("pointercancel", endDrag);
  track.addEventListener("dragstart", (e) => e.preventDefault()); // links and images
  // A drag must not end in a click that selects or opens a panel.
  track.addEventListener(
    "click",
    (e) => {
      if (!dragged) return;
      dragged = false;
      e.preventDefault();
      e.stopPropagation();
    },
    true,
  );

  track.addEventListener("keydown", (e) => {
    if (e.key === "Enter" && e.target.classList.contains("panel")) open(e.target);
  });

  doc.addEventListener("keydown", (e) => {
    if (ignoreKey(e)) return;
    if (e.key === "ArrowRight") step(1);
    else if (e.key === "ArrowLeft") step(-1);
    else return;
    e.preventDefault();
  });

  for (const d of dots) {
    d.addEventListener("click", () => select(panels.find((p) => p.dataset.id === d.dataset.id), true));
  }
  for (const c of chips) c.addEventListener("click", () => filter(c.dataset.filter, true));

  // Only hashes naming a filter count; #main (skip link) and junk leave the view alone.
  const fromHash = () => {
    let id = "";
    try {
      id = decodeURIComponent(location.hash.slice(1));
    } catch (_) {
      return;
    }
    if (id === "" || chips.some((c) => c.dataset.filter === id)) filter(id || "all", false);
  };
  addEventListener("hashchange", fromHash);
  fromHash();
})();
