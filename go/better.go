package main

type DummyArrayBetter struct {
	set      []uint
	values   []uint
	capacity uint
	counter  uint
}

func NewDummyArrayBetter(capacity uint) *DummyArrayBetter {
	return &DummyArrayBetter{
		set:      make([]uint, capacity),
		values:   make([]uint, capacity),
		capacity: capacity,
		counter:  0,
	}
}

func (da *DummyArrayBetter) Add(value uint) bool {
	if value >= da.capacity || da.Exists(value) {
		return false
	}

	return true
}

func (da *DummyArrayBetter) Remove(value uint) bool { return false }
func (da *DummyArrayBetter) Exists(value uint) bool {
	if value >= da.capacity {
		return false
	}

	return false //da.set[value]
}
