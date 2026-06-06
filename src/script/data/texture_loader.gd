@tool
class_name TextureLoader extends Node

var atlas: AtlasTexture
var registry: Registry

func _ready() -> void:
	registry = Registry.new()
