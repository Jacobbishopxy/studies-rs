package main

import "fmt"

type observer interface {
	update(state string)
}

// TextObserver 为 ConcreteObserver
type TextObserver struct {
	_name string
}

func (t TextObserver) update(state string) {
	fmt.Println(t._name + ": " + state)
}

// TestSubject 为 Subject
type TestSubject struct {
	_observers []observer
}

// Attach 增加一个 observer
func (ts *TestSubject) Attach(observer observer) {
	ts._observers = append(ts._observers, observer)
}

// Detach 移除一个 observer
func (ts *TestSubject) Detach(observer observer) {
	index := 0
	for i := range ts._observers {
		if ts._observers[i] == observer {
			index = i
			break
		}
	}
	ts._observers = append(ts._observers[0:index], ts._observers[index+1:]...)
}

func (ts TestSubject) notify(state string) {
	for _, observer := range ts._observers {
		observer.update(state)
	}
}

// TextEdit 为 ConcreteSubject
type TextEdit struct {
	TestSubject
	Text string
}

// SetText 改变 text 以及 通知 observers
func (te *TextEdit) SetText(text string) {
	te.Text = text
	te.TestSubject.notify(text)
}

func main() {
	// 客户端
	observer1 := TextObserver{"IObserver #1"}
	observer2 := TextObserver{"IObserver #2"}

	textEdit := TextEdit{}
	textEdit.Attach(observer1)
	textEdit.Attach(observer2)

	textEdit.SetText("test text")
}
