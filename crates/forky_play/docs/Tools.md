# Tools


## Selection Flow
```rs


if select_button
	if moused_over
		if primary_selected != moused_over
			remove(primary_selected,_primary_select)
			add(moused_over,primary_select)

		if !multi_select_button //select,moused_over,multi_select
			deselect_all
		add(moused_over,selected)
	else if !multi_select_button
		deselect_all()

if select_button && !moused_over
	deselect all
		
```

## Resources
