package bench

import (
	"time"

	"github.com/labian0/dummy-array/go/internal/dummyarray"
	"github.com/labian0/dummy-array/go/internal/timer"
)

func BenchmarkInitialize(capacity, repetitions uint) int64 {
	timer := timer.Timer{}
	var i uint
	var timespanSum int64 = 0
	for i = 0; i < repetitions; i++ {
		timer.Start()
		dummyarray.NewDummyArrayBetter(capacity)
		timer.End()
		timespanSum += timer.GetTimespanNS()
	}
	return timespanSum / int64(repetitions)
}

func BenchmarkAdd(capacity, repetitions uint) int64 {
	timer := timer.Timer{}
	var i uint
	var j uint
	var timespanSum int64 = 0
	da := dummyarray.NewDummyArrayBetter(capacity)
	for i = 0; i < repetitions; i++ {
		timer.Start()
		for j = 0; j < capacity; j++ {
			da.Add(uint(time.Now().Nanosecond()) % capacity)
		}
		timer.End()
		timespanSum += timer.GetTimespanNS()
	}
	return timespanSum / int64(repetitions)
}

func BenchmarkRemove(capacity, repetitions uint) int64 {
	timer := timer.Timer{}
	var i uint
	var j uint
	var timespanSum int64 = 0
	da := dummyarray.NewDummyArrayBetter(capacity)
	// remplissage du dummy array
	for i = 0; i < capacity; i++ {
		da.Add(i)
	}
	for i = 0; i < repetitions; i++ {
		timer.Start()
		for j = 0; j < capacity; j++ {
			da.Remove(uint(time.Now().Nanosecond()) % capacity)
		}
		timer.End()
		timespanSum += timer.GetTimespanNS()
	}
	return timespanSum / int64(repetitions)
}

func BenchmarkExists(capacity, repetitions uint) int64 {
	timer := timer.Timer{}
	var i uint
	var j uint
	var timespanSum int64 = 0
	da := dummyarray.NewDummyArrayBetter(capacity)
	// remplissage partiel du dummy array
	for i = 0; i < capacity/2; i++ {
		da.Add(uint(time.Now().Nanosecond()) % capacity)
	}
	for i = 0; i < repetitions; i++ {
		timer.Start()
		for j = 0; j < capacity; j++ {
			da.Exists(j)
		}
		timer.End()
		timespanSum += timer.GetTimespanNS()
	}
	return timespanSum / int64(repetitions)
}
