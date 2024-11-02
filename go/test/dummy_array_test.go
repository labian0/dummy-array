package main

import (
	"fmt"
	"testing"

	"github.com/labian0/dummy-array/go/internal/dummyarray"
)

// check for a DummyArray of capacity 10
func DummyArrayCheck(constructor func(uint) dummyarray.DummyArray) error {
	da := constructor(10)
	if da.Add(128) {
		return fmt.Errorf("nombre trop grand entré avec succès dans le DA")
	}
	if da.Add(10) {
		return fmt.Errorf("nombre égal à la capacité entré avec succès dans le DA")
	}
	if da.Exists(128) {
		return fmt.Errorf("nombre trop grand présent dans le DA")
	}
	if da.Exists(10) {
		return fmt.Errorf("nombre trop grand présent dans le DA")
	}
	if da.Exists(1) {
		return fmt.Errorf("nombre non ajouté présent dans le DA")
	}
	if !da.Add(2) {
		return fmt.Errorf("nombre valide non ajouté au DA")
	}
	if !da.Exists(2) {
		return fmt.Errorf("nombre ajouté absent du DA")
	}
	return nil
}

func TestDummyArrayNaive(t *testing.T) {
	err := DummyArrayCheck(func(u uint) dummyarray.DummyArray {
		return dummyarray.NewDummyArrayNaive(u)
	})
	if err != nil {
		t.Fatal(err.Error())
	}
}
func TestDummyArrayBetter(t *testing.T) {
	err := DummyArrayCheck(func(u uint) dummyarray.DummyArray {
		return dummyarray.NewDummyArrayBetter(u)
	})
	if err != nil {
		t.Fatal(err.Error())
	}
}

func doStuffToDummyArray(constructor func(uint) dummyarray.DummyArray) {
	//initialization
	const CAPACITY uint = 1000000
	da := constructor(CAPACITY)
	var value uint
	var i uint
	//adding
	value = 0
	for i = 0; i < CAPACITY; i++ {
		da.Add(value)
		value = (value + 3) % CAPACITY
	}
	//removing
	value = 0
	for i = 0; i < CAPACITY; i++ {
		da.Remove(value)
		value = (value + 3) % CAPACITY
	}
	//checking exists
	value = 0
	for i = 0; i < CAPACITY; i++ {
		da.Exists(value)
		value = (value + 3) % CAPACITY
	}
}

func BenchmarkDummyArrayNaive(b *testing.B) {
	for i := 0; i < b.N; i++ {
		doStuffToDummyArray(func(u uint) dummyarray.DummyArray { return dummyarray.NewDummyArrayNaive(u) })
	}
}
func BenchmarkDummyArrayBetter(b *testing.B) {
	for i := 0; i < b.N; i++ {
		doStuffToDummyArray(func(u uint) dummyarray.DummyArray { return dummyarray.NewDummyArrayBetter(u) })
	}
}
