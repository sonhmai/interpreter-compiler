package object

type ObjectType string

// Object represents language object system.
// Every value in the language must implement this interface
type Object interface {
	Type() ObjectType
	Inspect() string
}
