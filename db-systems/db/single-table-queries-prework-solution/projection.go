package solution

// ProjectionOperator returns tuples in which only the specified fields are contained.
type ProjectionOperator struct {
	fields map[string]struct{}
	child  Operator
}

// NewProjectionOperator creates a new ProjectionOperator.
func NewProjectionOperator(fields map[string]struct{}, child Operator) Operator {
	return &ProjectionOperator{
		fields: fields,
		child:  child,
	}
}

// Next returns a boolean indicating whether there are more tuples to select.
func (s *ProjectionOperator) Next() bool {
	return s.child.Next()
}

// Execute returns the next Tuple with only the specified values selected.
func (s *ProjectionOperator) Execute() Tuple {
	tuple := s.child.Execute()

	// Filter in place to avoid an allocation.
	filtered := tuple.Values[:0]
	for _, v := range tuple.Values {
		if _, ok := s.fields[v.Name]; ok {
			filtered = append(filtered, v)
		}
	}
	tuple.Values = filtered

	return tuple
}
