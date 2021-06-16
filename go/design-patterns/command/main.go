package main

import "fmt"

// Command 类型
type Command interface {
	Execute()
}

// BankClient 作为调用者
type BankClient struct {
	putCommand Command
	getCommand Command
}

// PutMoney 运行 putCommand
func (bc BankClient) PutMoney() {
	bc.putCommand.Execute()
}

// GetMoney 运行 getCommand
func (bc BankClient) GetMoney() {
	bc.getCommand.Execute()
}

// Bank 作为接收者
type Bank struct{}

func (b Bank) giveMoney() {
	fmt.Println("money to the client")
}

func (b Bank) receiveMoney() {
	fmt.Println("money from the client")
}

// PutCommand 是具体 Command
type PutCommand struct {
	bank Bank
}

// 执行 command
func (pc PutCommand) Execute() {
	pc.bank.receiveMoney()
}

// GetCommand 是具体 Command
type GetCommand struct {
	bank Bank
}

// 执行 command
func (gc GetCommand) Execute() {
	gc.bank.giveMoney()
}

// 测试函数
func main() {
	// 客户端
	bank := Bank{}
	cPut := PutCommand{bank}
	cGet := GetCommand{bank}
	client := BankClient{cPut, cGet}

	client.GetMoney()
	client.PutMoney()
}
