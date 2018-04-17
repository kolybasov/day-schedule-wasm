window.renderEvents = (function(document) {
  'use strict';
  /**
   * ===========================
   * Schedule
   * ===========================
   *
   */
  function Schedule(options = {}) {
    this.width = options.width;
    this.height = options.height;
    this.eventWidth = this.width - options.padding * 2;
    this.hourHeight = this.height / (Schedule.hours.length - 1);
    this._clipPathId = 0;

    this.createContainer();
    this.renderGrid();
    this.createEventsContainer();
  }

  Schedule.hours = [
    '9 AM',
    '10 AM',
    '11 AM',
    '12 PM',
    '1 PM',
    '2 PM',
    '3 PM',
    '4 PM',
    '5 PM',
    '6 PM',
    '7 PM',
    '8 PM',
    '9 PM'
  ];

  Schedule.prototype = {
    // Crate SVG container for schedule
    createContainer() {
      this.container = this._createEl('svg', {
        width: this.width,
        height: this.height,
        viewBox: `0 0 ${this.width} ${this.height}`,
        style: `
          overflow: visible;
          font-family: Helvetica, Arial;
          font-size: 0.8em;
        `
      });
    },

    // Create separate group for events
    createEventsContainer() {
      this.eventsContainer = this._createEl('g', {
        transform: 'translate(10,0)'
      });
      this.container.appendChild(this.eventsContainer);
    },

    // Clear previous events before rendering a new one
    clearEventsContainer() {
      while (this.eventsContainer.firstChild) {
        this.eventsContainer.removeChild(this.eventsContainer.firstChild);
      }
    },

    // Render grid to SVG
    renderGrid() {
      let g = this._createEl('g', {
        stroke: '#f8f8f8'
      });

      let hours = Schedule.hours;
      for (let i = 0, len = hours.length; i < len; i++) {
        this._renderHourGroup(g, hours[i], i * this.hourHeight);
      }

      this._renderVLine(g, this.width);
      this._renderVLine(g, 0);

      this.container.appendChild(g);
    },

    // Render event to events container
    renderEvent(event, widthMultiplier, shift) {
      if (!event || event.placeholder) {
        return;
      }
      let width = 1 / widthMultiplier * this.eventWidth;
      let height = this.hourHeight / 60 * event.duration;

      let clipPath = this._createClipPath({ width, height });
      let g = this._createEl('g', {
        transform: `translate(${shift * width},${event.starts_at})`,
        'clip-path': `url(#${clipPath.id})`
      });

      let rect = this._createEl('rect', {
        width,
        height,
        fill: '#45a51c',
        opacity: 0.3
      });
      g.appendChild(rect);

      let line = this._createEl('line', {
        'stroke-width': 4,
        stroke: '#45a51c',
        y2: height
      });
      g.appendChild(line);

      let text = this._createEl('text', {
        transform: `translate(10,${height / 2})`,
        fill: '#45a51c'
      });
      let result = this._renderEventTitle(text, event);
      this._renderEventTime(text, event, result !== 'none');
      g.appendChild(text);

      this.eventsContainer.appendChild(g);
    },

    // Create clip path. Is used to cut text if it overflows container
    _createClipPath(opts) {
      this._clipPathId++;
      let id = `clip-path-${this._clipPathId}`;
      let el = this._createEl('clipPath', { id });
      let rect = this._createEl('rect', {
        width: opts.width,
        height: opts.height
      });
      el.appendChild(rect);
      this.container.appendChild(el);
      return { id, el };
    },

    // Render title to event
    _renderEventTitle(container, event) {
      if (!event.title && !event.location) {
        return 'none';
      }

      let title = this._createEl('tspan', {
        style: 'font-weight: bold;',
        dy: '-0.1em'
      });

      let titleStr = event.title || '';
      if (titleStr !== '' && event.location) {
        titleStr = `${titleStr}/${event.location}`;
      } else if (event.location) {
        titleStr = event.location;
      }

      title.textContent = titleStr;
      container.appendChild(title);
    },

    // Render time to event
    _renderEventTime(container, event, shiftText) {
      let time = this._createEl('tspan', {
        dy: `${shiftText ? 0.9 : 0.35}em`,
        x: 0
      });
      time.textContent = this._formatEventTime(event);
      container.appendChild(time);
    },

    // Format time from interval to human readable
    _formatEventTime(event) {
      let startTime = this._formatInterval(event.starts_at);
      let endTime = this._formatInterval(event.starts_at + event.duration);
      return `${startTime} – ${endTime}`;
    },

    // Format interval to human readable
    _formatInterval(time) {
      let minute = time % 60;
      let hour = (time - minute) / 60 + 9;
      let period = hour < 12 ? 'AM' : 'PM';
      let result = `${hour > 12 ? hour - 12 : hour}`;
      if (minute !== 0) {
        result = `${result}:${minute}`;
      }
      result = `${result} ${period}`;
      return result;
    },

    // Render hour text and line to grid
    _renderHourGroup(container, hText, offset) {
      let g = this._createEl('g', {
        transform: `translate(0,${offset})`
      });

      let line = this._createEl('line', {
        x2: this.width
      });

      let text = this._createEl('text', {
        'text-anchor': 'end',
        dy: '.35em',
        x: -10,
        fill: '#c9c9c9',
        stroke: 'none'
      });
      text.textContent = hText;

      g.appendChild(line);
      g.appendChild(text);
      container.appendChild(g);
    },

    // Render grid vertical line
    _renderVLine(container, offset) {
      let vLine = this._createEl('line', {
        transform: `translate(${offset},0)`,
        y2: this.height
      });
      container.appendChild(vLine);
    },

    // Create namespaced element
    _createEl(name, attrs = {}) {
      let el = document.createElementNS('http://www.w3.org/2000/svg', name);
      for (let attr in attrs) {
        el.setAttributeNS(null, attr, attrs[attr]);
      }
      return el;
    }
  };
  /**
   * ===========================
   * renderEvents
   * ===========================
   */
  let schedule;
  function renderEvents(events, container) {
    if (!container) {
      container = document.body;
    }

    // Create container if it is a first call
    if (!schedule) {
      schedule = new Schedule({
        width: 600,
        height: 720,
        padding: 10
      });
    }

    // Clear old events before rendering a new one
    schedule.clearEventsContainer();

    // Render each event
    let positions = calculate_positions(events);
    for (let posIdx = 0, pl = positions.length; posIdx < pl; posIdx++) {
      let position = positions[posIdx];
      schedule.renderEvent(
        events[position.id],
        position.width_multiplier,
        position.offset
      );
    }

    // And append result
    container.appendChild(schedule.container);
  }

  return renderEvents;
})(document);
