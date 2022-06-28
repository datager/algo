package main

func main() {
	arr := []int64{1, 3, 4, 5, 6, 8, 8, 8, 11, 18}
	for _, a := range arr {
		println("值:", a, "下标:", bSearchLastLte(arr, a))
	}
}

// bSearchAnyEqual 二分查找任意等于元素的下标
func bSearchAnyEqual[V Value](arr []V, v V) int {
	sz := len(arr)
	low := 0
	high := sz - 1
	for low <= high {
		mid := (low + high) / 2
		if arr[mid] == v {
			return mid
		} else if arr[mid] < v {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return -1
}

type Value interface {
	int64 | string
}

// bSearchFirstEqual 二分查找第一个等于元素的下标
func bSearchFirstEqual[V Value](arr []V, v V) int {
	sz := len(arr)
	low := 0
	high := sz - 1
	for low <= high {
		mid := (low + high) / 2
		if arr[mid] == v {
			if mid == 0 || arr[mid-1] != v {
				return mid
			} else {
				high = mid - 1
			}
		} else if arr[mid] < v {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return -1
}

// bSearchLastEqual 二分查找最后一个等于元素的下标
func bSearchLastEqual[V Value](arr []V, v V) int {
	sz := len(arr)
	low := 0
	high := sz - 1
	for low <= high {
		mid := (low + high) / 2
		if arr[mid] == v {
			if mid == sz-1 || arr[mid+1] != v {
				return mid
			} else {
				low = mid + 1
			}
		} else if arr[mid] < v {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return -1
}

// bSearchFirstGte 二分查找第一个大于等于元素的下标
func bSearchFirstGte[V Value](arr []V, v V) int {
	sz := len(arr)
	low := 0
	high := sz - 1
	for low <= high {
		mid := (low + high) / 2
		if arr[mid] >= v {
			if mid == 0 || arr[mid-1] < v {
				return mid
			} else {
				high = mid - 1
			}
		} else {
			low = mid + 1
		}
	}
	return -1
}

// bSearchLastLte 二分查找最后一个小于等于元素的下标
func bSearchLastLte[V Value](arr []V, v V) int {
	sz := len(arr)
	low := 0
	high := sz - 1
	for low <= high {
		mid := (low + high) / 2
		if arr[mid] <= v {
			if mid == sz-1 || arr[mid+1] > v {
				return mid
			} else {
				low = mid + 1
			}
		} else {
			high = mid - 1
		}
	}
	return -1
}
