package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	// gin router 初始化
	router := gin.Default()
	// index 路由
	router.GET("/", func(context *gin.Context) {
		context.JSON(http.StatusOK, gin.H{"data": "Hello World!"})
	})
	// 启动服务
	router.Run(":8080")
}
