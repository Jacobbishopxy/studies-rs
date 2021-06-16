package main

// IntIterator 是遍历器
type IntIterator interface {
	First()
	Next()
	IsDone() bool
	CurrentItem() int
}

// Numbers 作为 ConcreteAggregate
type Numbers struct {
	Data []int
}

// GetIterator 返回遍历器
func (n Numbers) GetIterator() IntIterator {
	return &Iterator{n, 0}
}

// Iterator 作为 ConcreteIterator
type Iterator struct {
	_numbers Numbers
	_index   int
}

// 首位是遍历器的第一个元素
func (i *Iterator) First() {
	i._index = 0
}

// Next 推进当前元素
func (i *Iterator) Next() {
	i._index++
}

// IsDone 检查索引是否指向 List 中的元素
func (i *Iterator) IsDone() bool {
	return i._index >= len(i._numbers.Data)
}

// CurrentItem 返回当前索引的元素
func (i *Iterator) CurrentItem() int {
	return i._numbers.Data[i._index]
}

func main() {
	// 客户端
	numbers := Numbers{[]int{2, 3, 5, 7, 11}}
	iterator := numbers.GetIterator()
	sum := 0
	for iterator.First(); !iterator.IsDone(); iterator.Next() {
		sum += iterator.CurrentItem()
	}
}
