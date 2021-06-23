package main

import "fmt"

// 作为 Component
type Graphic interface {
	Draw()
}

// 作为 Leaf
type Circle struct{}

// 作为 Operation
func (c Circle) Draw() {
	fmt.Println("Draw circle")
}

// 作为 Leaf
type Square struct{}

// 作为 Operation
func (s Square) Draw() {
	fmt.Println("Draw square")
}

// 作为 Composite
type Image struct {
	graphics []Graphic
}

// 添加 Leaf 给 Composite
func (i *Image) Add(graphic Graphic) {
	i.graphics = append(i.graphics, graphic)
}

// 作为 Operation
func (i Image) Draw() {
	fmt.Println("Draw image")
	for _, g := range i.graphics {
		g.Draw()
	}
}

func main() {
	image := Image{}
	image.Add(Circle{})
	image.Add(Square{})
	picture := Image{}
	picture.Add(image)
	picture.Add(Image{})
	picture.Draw()
}
