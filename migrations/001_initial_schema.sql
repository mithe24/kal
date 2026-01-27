CREATE TABLE calendars (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    is_archived INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_calendars_archived
    ON calendars (is_archived);


/* Only for nonrecurrent events */
CREATE TABLE events (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    starts_at TEXT NOT NULL,
    ends_at TEXT NOT NULL,
    color INTEGER NOT NULL DEFAULT 0,
    is_all_day INTEGER NOT NULL DEFAULT 0,
    is_cancelled INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (calendar_id)
        REFERENCES calendars(id)
        ON DELETE CASCADE,
    CHECK (starts_at < ends_at),
    CHECK (color BETWEEN 0 AND 255)
);

CREATE TABLE recurrences (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    starts_at TEXT NOT NULL,
    ends_at TEXT NOT NULL,
    frequency TEXT NOT NULL,
    interval INTEGER NOT NULL,
    until TEXT,
    color INTEGER NOT NULL DEFAULT 0,
    is_all_day INTEGER NOT NULL DEFAULT 0,
    is_cancelled INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (calendar_id)
        REFERENCES calendars(id)
        ON DELETE CASCADE,
    CHECK (starts_at < ends_at),
    CHECK (interval > 0),
    CHECK (color BETWEEN 0 AND 255)
);

CREATE TABLE recurrence_exceptions (
    recurrence_id TEXT NOT NULL,
    original_starts_at TEXT NOT NULL,
    new_starts_at TEXT,
    new_ends_at TEXT,
    is_cancelled INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (recurrence_id, original_starts_at)
    FOREIGN KEY (recurrence_id)
        REFERENCES recurrences(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_events_overlap
    ON events (calendar_id, ends_at, starts_at)
    WHERE is_cancelled = 0;

CREATE INDEX idx_events_active
    ON events (calendar_id, is_cancelled, starts_at);

CREATE INDEX idx_events_calendar_dates
    ON events (calendar_id, starts_at, ends_at)
    WHERE is_cancelled = 0;

CREATE INDEX idx_recurrences_calendar
    ON recurrences (calendar_id);

CREATE INDEX idx_recurrences_start_end
    ON recurrences (starts_at, ends_at);

CREATE INDEX idx_recurrences_active
    ON recurrences (is_cancelled, until);

CREATE INDEX idx_recurrence_exceptions_recurrence
    ON recurrence_exceptions (recurrence_id);

CREATE INDEX idx_recurrence_exceptions_original_date
    ON recurrence_exceptions (recurrence_id, original_starts_at);
