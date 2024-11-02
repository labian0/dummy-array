package timer

import "time"

type Timer struct {
	startTimestamp time.Time
	endTimestamp   time.Time
	timespanNS     int64 // in nanoseconds
}

func (t *Timer) computeTimespan() {
	t.timespanNS = t.endTimestamp.UnixNano() - t.startTimestamp.UnixNano()
}

// Start time recording.
func (t *Timer) Start() {
	t.startTimestamp = time.Now()
}

// End time recording.
func (t *Timer) End() {
	t.endTimestamp = time.Now()
	t.computeTimespan()
}

// Returns the timespan in nanoseconds (10^9*seconds)
func (t *Timer) GetTimespanNS() int64 {
	return t.timespanNS
}
