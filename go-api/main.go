package main

import (
	"context"
	"fmt"
	"strconv"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func Handler(ctx context.Context, request events.APIGatewayProxyRequest) (string, error) {

	num := request.QueryStringParameters["n"]
	n, _ := strconv.ParseInt(num, 10, 64)
	return fmt.Sprintf("Final count is %d", counter(n)), nil
}

func counter(n int64) int64 {
	count := int64(0)

	if n > 5000000000 {
		n = 5000000000
	}

	for i := int64(0); i <= n; i++ {
		count += i
	}

	return count
}
func main() {
	lambda.Start(Handler)
}
