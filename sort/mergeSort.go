package main

type mergeSort struct {
	arr []int64
}

func NewMergeSort(arr []int64) *mergeSort {
	return &mergeSort{arr: arr}
}

func (s *mergeSort) Sort() []int64 {
	return s.sort(s.arr)
}

func (s *mergeSort) sort(arr []int64) []int64 {
	sz := len(arr)
	if sz == 1 {
		return arr
	}
	mid := (1 + sz) / 2
	arr1 := s.sort(arr[:mid])
	arr2 := s.sort(arr[mid:])
	return s.merge(arr1, arr2)
}

func (s *mergeSort) merge(arr1, arr2 []int64) []int64 {
	sz1 := len(arr1)
	sz2 := len(arr2)
	arr := make([]int64, sz1+sz2, sz1+sz2)
	i := 0
	k := 0
	total := 0
	for i < sz1 && k < sz2 {
		if arr1[i] < arr2[k] {
			arr[total] = arr1[i]
			i++
		} else {
			arr[total] = arr2[k]
			k++
		}
		total++
	}
	return arr
}
