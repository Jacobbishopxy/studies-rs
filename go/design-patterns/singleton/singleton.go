package main

import "fmt"

// 简单结构体
type Settings struct {
	Port int
	Host string
}

var instance *Settings

// 该函数返回一个单独的实例
func GetInstance() *Settings {
	if instance == nil {
		instance = &Settings{} // 非线程安全
	}
	return instance
}

func main() {
	settings := GetInstance()

	settings.Host = "192.168.100.1"
	settings.Port = 33

	settings1 := GetInstance()
	//settings1.Port is 33

	fmt.Println(settings1)
}
