package main

import "fmt"

// 抽象的 Product
type Employee interface {
	Test()
}

// 具体的 Product
type Booker struct{}

// 为 Booker 实现 Employee 方法
func (e Booker) Test() {
	fmt.Println("Booker")
}

// 具体的 Product
type Manager struct{}

// 为 Manager 实现 Employee 方法
func (e Manager) Test() {
	fmt.Println("Manager")
}

// 具体的 Creator
type BookerCreator struct{}

// BookerCreator 的 CreateEmployee 方法
func (c BookerCreator) CreateEmployee() Employee {
	return Booker{}
}

// 具体的 Creator
type ManagerCreator struct{}

// ManagerCreator 的 CreateEmployee 方法
func (c ManagerCreator) CreateEmployee() Employee {
	return Manager{}
}

func main() {
	booker := BookerCreator{}.CreateEmployee()
	booker.Test()
	//printed: Booker

	manager := ManagerCreator{}.CreateEmployee()
	manager.Test()
	//printed: Manager
}
