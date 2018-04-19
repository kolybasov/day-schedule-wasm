const rust = import('events_matrix/events_matrix');

window.renderEvents = (function(document) {
  'use strict';

  let wrapper;
  function renderEvents(events, container) {
    if (!container) { container = document.body; }
    if (!wrapper) {
      wrapper = document.createElement('div');
      container.appendChild(wrapper);
    }

    let eventsStr = JSON.stringify(events);
    return rust.then(m => {
      let svg = m.render_events(eventsStr);
      wrapper.innerHTML = svg;
    });
  }

  return renderEvents;
})(document);
