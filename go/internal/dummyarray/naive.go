package dummyarray

/*
DummyArrayNaive: utilise un seul set (map[uint]bool)
extrêmement simple
non-compliant avec la consigne (il faut deux tableaux (qui peuvent être des map))
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
	if value >= da.capacity || da.Exists(value) {
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
