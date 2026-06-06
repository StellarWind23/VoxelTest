class_name MFace extends Resource

@export var uv: Vector2i
@export var texture: Id
@export var cullface: CullFace

enum CullFace {
	UP,
	DOWN,
	NORTH,
	SOUTH,
	EAST,
	WEST
}
