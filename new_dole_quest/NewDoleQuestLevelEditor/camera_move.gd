extends Node2D

const ZOOM_SENSITIVITY: float = 0.125

var camera: Camera2D = null
var can_move: bool = false
var delt: float = 0.0

func _ready():
	camera = $Camera2D

func _input(event):
	if event.is_action_pressed("zoom_in"):
		camera.zoom += Vector2.ONE * ZOOM_SENSITIVITY
	elif event.is_action_pressed("zoom_out"):
		var current_zoom: Vector2 = camera.zoom - (Vector2.ONE * ZOOM_SENSITIVITY)
		current_zoom = Vector2(max(current_zoom.x, 0.01), max(current_zoom.y, 0.01))
		camera.zoom = current_zoom

func _unhandled_input(event):
	can_move = Input.is_action_pressed("click")
	if event is InputEventMouseMotion && can_move:
		camera.position += -event.relative / camera.zoom
