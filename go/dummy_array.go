package main

type DummyArray interface {
	Add(uint) bool
	Remove(uint) bool
	Exists(uint) bool
}

/*
DummyArrayNaive: utilise un seul set (map[uint]bool)
extrêmement simple
*/

type DummyArrayNaive struct {
	set      map[uint]bool
	capacity uint
}

func NewDummyArrayNaive(capacity uint) *DummyArrayNaive {
	da := DummyArrayNaive{
		capacity: capacity,
		set:      make(map[uint]bool, capacity),
	}
	return &da
}

func (da *DummyArrayNaive) Add(value uint) bool {
	if value >= da.capacity {
		return false
	}
	da.set[value] = true
	return true
}

func (da *DummyArrayNaive) Remove(value uint) bool {
	if value >= da.capacity {
		return false
	}
	if !da.set[value] {
		return false
	}
	da.set[value] = false
	return true
}

func (da *DummyArrayNaive) Exists(value uint) bool {
	return da.set[value]
}
