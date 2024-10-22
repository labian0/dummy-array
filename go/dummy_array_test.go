package main

import (
	"fmt"
	"testing"
)

// check for a DummyArray of capacity 10
func DummyArrayCheck(constructor func(uint) DummyArray) error {
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
	err := DummyArrayCheck(func(u uint) DummyArray {
		return NewDummyArrayNaive(u)
	})
	if err != nil {
		t.Fatal(err.Error())
	}
}
func TestDummyArrayBetter(t *testing.T) {
	err := DummyArrayCheck(func(u uint) DummyArray {
		return NewDummyArrayBetter(u)
	})
	if err != nil {
		t.Fatal(err.Error())
	}
}

func doStuffToDummyArray(constructor func(uint) DummyArray) {
	//initialization
	da := constructor(1000000)
	var value uint
	//adding
	value = 0
	for i := 0; i < 1000000; i++ {
		da.Add(value)
		value = (value + 3) % 1000000
	}
	//removing
	value = 0
	for i := 0; i < 1000000; i++ {
		da.Remove(value)
		value = (value + 3) % 1000000
	}
	//checking exists
	value = 0
	for i := 0; i < 1000000; i++ {
		da.Exists(value)
		value = (value + 3) % 1000000
	}
}

func BenchmarkDummyArrayNaive(b *testing.B) {
	for i := 0; i < b.N; i++ {
		doStuffToDummyArray(func(u uint) DummyArray { return NewDummyArrayNaive(u) })
	}
}
func BenchmarkDummyArrayBetter(b *testing.B) {
	for i := 0; i < b.N; i++ {
		doStuffToDummyArray(func(u uint) DummyArray { return NewDummyArrayBetter(u) })
	}
}
