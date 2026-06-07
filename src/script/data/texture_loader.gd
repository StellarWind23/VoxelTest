@tool
class_name TextureLoader extends Node

#Block
var block_atlas: AtlasTexture
var block_registry: Registry
#Item
var item_atlas: AtlasTexture
var item_registry: Registry
#Particle
var particle_atlas: AtlasTexture
var particle_registry: Registry
#GUI
var gui_atlas: AtlasTexture
var gui_registry: Registry
#Entity
var entity_atlas: AtlasTexture
var entity_registry: Registry

static var _instance: TextureLoader

func instance():
	return self._instance

func _ready() -> void:
	#Block
	block_registry = Registry.new()
	block_atlas = AtlasTexture.new()
	#Block
	item_registry = Registry.new()
	item_atlas = AtlasTexture.new()
	#particle
	particle_registry = Registry.new()
	particle_atlas = AtlasTexture.new()
	#gui
	gui_registry = Registry.new()
	gui_atlas = AtlasTexture.new()
	#entity
	entity_registry = Registry.new()
	entity_atlas = AtlasTexture.new()
	
	
	#Set instance
	_instance = self
	
	#Load png files.
