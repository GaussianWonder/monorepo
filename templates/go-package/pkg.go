package {{ name }}

import "fmt"

func Hello(name string) string {
	fmt.Println("within {{ name }}")
	return fmt.Sprintf("Hello, %s! This is {{ name }}.", name)
}
