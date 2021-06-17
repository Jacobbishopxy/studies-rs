package main

import "fmt"

// Mediator 定义一个作用于 Colleague 对象之间交流的接口
type Mediator interface {
	Sync(switcher *Switcher)
	Add(switcher *Switcher)
}

// Switcher 为 Colleague
type Switcher struct {
	State     bool
	_mediator Mediator
}

// Switcher 构造函数
func NewSwitcher(mediator Mediator) *Switcher {
	switcher := &Switcher{false, mediator}
	mediator.Add(switcher)
	return switcher
}

// Sync 启动 mediator 的 Sync 函数
func (s Switcher) Sync() {
	s._mediator.Sync(&s)
}

// SyncMediator 为具体 Mediator
type SyncMediator struct {
	Switchers []*Switcher
}

// Sync 同步所有 Colleague 对象的 state
func (sm *SyncMediator) Sync(switcher *Switcher) {
	for _, curSwitcher := range sm.Switchers {
		curSwitcher.State = switcher.State
	}
}

// 添加 Colleague 至 Mediator 数组
func (sm *SyncMediator) Add(switcher *Switcher) {
	sm.Switchers = append(sm.Switchers, switcher)
}

func main() {
	// 客户端
	mediator := &SyncMediator{[]*Switcher{}}
	switcher1 := NewSwitcher(mediator)
	switcher2 := NewSwitcher(mediator)
	switcher3 := NewSwitcher(mediator)

	switcher1.State = true
	state2 := switcher2.State
	fmt.Println(state2)
	state3 := switcher3.State
	fmt.Println(state3)

	switcher1.Sync()
	state2 = switcher2.State
	fmt.Println(state2)
	state3 = switcher3.State
	fmt.Println(state3)
}
