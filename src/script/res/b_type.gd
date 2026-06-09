@tool
class_name BType extends Resource

@export_category("Basic")
@export var block_state: State
@export var ao_color: Color
@export var break_time: int
@export var e_resist: int
@export var friction: float
@export var bounciness: float
@export var sounds: Dictionary[Id, Id]
@export_category("Advanced")
@export var events: Dictionary
