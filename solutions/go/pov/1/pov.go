package pov

type Tree struct {
	value    string
	children []*Tree
}

// New creates and returns a new Tree with the given root value and children.
func New(value string, children ...*Tree) *Tree {
	return &Tree{value, children}
}

// Value returns the value at the root of a tree.
func (tr *Tree) Value() string {
	return tr.value
}

// Children returns a slice containing the children of a tree.
// There is no need to sort the elements in the result slice,
// they can be in any order.
func (tr *Tree) Children() []*Tree {
	return tr.children
}

// String describes a tree in a compact S-expression format.
// This helps to make test outputs more readable.
// Feel free to adapt this method as you see fit.
func (tr *Tree) String() string {
	if tr == nil {
		return "nil"
	}
	result := tr.Value()
	if len(tr.Children()) == 0 {
		return result
	}
	for _, ch := range tr.Children() {
		result += " " + ch.String()
	}
	return "(" + result + ")"
}

// POV problem-specific functions

// FromPov returns the pov from the node specified in the argument.
func (tr *Tree) FromPov(from string) *Tree {
	_, root := tr.fromPovInternal(from)
	return root
}

func (tr *Tree) fromPovInternal(from string) (*Tree, *Tree) {
	if from == tr.value {
		return tr, tr
	}
	var tested []*Tree
	defer func() {
		if tested != nil {
			tr.children = append(tr.children, tested...)
		}
	}()
	for len(tr.children) > 0 {
		child := tr.children[0]
		tr.children = tr.children[1:]
		node, root := child.fromPovInternal(from)
		if node != nil {
			node.children = append(node.children, tr)
			return tr, root
		} else {
			tested = append(tested, child)
		}
	}
	return nil, nil
}

// PathTo returns the shortest path between two nodes in the tree.
func (tr *Tree) PathTo(from, to string) []string {
	tr = tr.FromPov(to)
	if tr == nil {
		return nil
	}
	path := tr.pathTo(from)
	if path == nil {
		return nil
	}
	return append(path, to)
}

func (tr *Tree) pathTo(from string) []string {
	if tr.value == from {
		return []string{}
	}

	for _, child := range tr.children {
		path := child.pathTo(from)
		if path != nil {
			return append(path, child.value)
		}
	}
	return nil
}
