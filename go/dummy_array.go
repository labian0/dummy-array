package main

type DummyArray interface {
	Add(uint) bool
	Remove(uint) bool
	Exists(uint) bool
}
