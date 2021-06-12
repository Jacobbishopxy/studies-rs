package main

import (
	"blog/infrastructure"
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	// gin router 初始化
	router := gin.Default()
	// index 路由
	router.GET("/", func(context *gin.Context) {
		// 加载 env
		infrastructure.LoadEnv()
		// 数据库练级
		infrastructure.NewDatabase()

		context.JSON(http.StatusOK, gin.H{"data": "Hello World!"})
	})
	// 启动服务
	router.Run(":8080")
}
