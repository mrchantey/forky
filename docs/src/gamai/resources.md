# Resources
- [Modular AI](https://youtu.be/IvK0ZlNoxjw)
- Chris Hecker - [Structure Vs Style](https://youtu.be/4eQp8SdzOa0)
- Kevin Dill, Dave Mark - [Improving AI Decision Modeling Through Utility Theory](https://www.gdcvault.com/play/1012410/Improving-AI-Decision-Modeling-Through)
- Kevin Dill, Dave Mark - [Embracing the Dark Art of Mathematical Modeling in Game AI](https://www.gdcvault.com/play/1015683/Embracing-the-Dark-Art-of)
- Kevin Dill - [Introducing GAIA](https://www.sisostds.org/DesktopModules/Bring2mind/DMX/Download.aspx?Command=Core_Download&EntryId=35466&PortalId=0&TabId=105)
- C++ Library - [BehaviorTree.cpp](https://www.behaviortree.dev/)



## Examples

Sniper Planner
- method: first valid option
- options:
 - Take Shot
	- considerations
		- Execution History - When the last shot was taken update this sensors 'next_shot' between 60 and 120. If elapsed time less than this, veto the actions.
		- Pick Target - Check for a valid enemy to pick, if no valid enemy, veto the action, otherwise write target to blackboard.
		- Pick Line of Retreat - Check area surrounding self, if no valid retreat, veto the action
		- Check shots fired - Basic curve, number of shots fired decreases likelyhood of next shot.
	- actions
		- Increment shots fired
		- Fire at target
 - Retreat
	- 
	- Pick Line of Retreat - if no retreat, veto action


```xml
<take_shot>
	<factors>
		<range_throttle min="60" max="120"/>
		<pick_target/>
		<pick_line_of_retreat weight="100"/>
	</factors>
	<actions>
		<increment_shots_fired/>
		<fire_at_target/>
	</actions>
</take_shot>
```