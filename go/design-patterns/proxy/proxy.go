package main

import "fmt"

// 作为 Subject
type Graphic interface {
	Draw()
}

// 作为 RealSubject
type Image struct {
	FileName string
}

// 作为 Request
func (im Image) Draw() {
	fmt.Println("draw " + im.FileName)
}

// 作为 Proxy
type ImageProxy struct {
	FileName string
	_image   *Image
}

// 创建一个 Subject
func (ip ImageProxy) GetImage() *Image {
	if ip._image == nil {
		ip._image = &Image{ip.FileName}
	}
	return ip._image
}

// 作为 Request
func (ip ImageProxy) Draw() {
	ip.GetImage().Draw()
}

func main() {
	proxy := ImageProxy{FileName: "1.png"}
	// 操作没有创建一个真实 Subject
	fileName := proxy.FileName
	// 转发给真实 Subject
	proxy.Draw()

	fmt.Println(fileName)
}
