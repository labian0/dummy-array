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
	da.values[da.counter] = value
	da.set[value] = da.counter
	da.counter++

	return true
}

func (da *DummyArrayBetter) Remove(value uint) bool {
	if !da.Exists(value) {
		return false
	}
	da.set[value] = (da.set[value] + 1) % da.capacity // value index in the values slice

	return true
}

func (da *DummyArrayBetter) Exists(value uint) bool {
	if value >= da.capacity {
		return false
	}
	return da.values[da.set[value]] == value
}
