extends Node2D

@export var tile_map_name: String = "TileMap"
@export var collision_map_name: String = "CollisionMap"
@export var path: String = "C:\\Users\\foreskin\\Desktop\\sum_good_shit\\programming\\rust\\practice_game\\new_dole_quest\\assets\\maps\\"
@export var map_name: String = "map"
@export var map_extension: String = ".map2d"

var tile_map: TileMap = null
var collision_map: TileMap = null

# This will act as our "main" function
# We want to export a map with IDs at each coordinate -- Vec2:cell_id
# We want to export a collision map as well -- merge collision shapes together
# Export Header file with ID information per map -- id:texture_path
# Let Godot do a bunch of work for us ;p
func _ready():
	tile_map = get_node(tile_map_name)
	collision_map = get_node(collision_map_name)
	if !is_directory_good():
		printerr("Cannot find: ", path)
		return

	var captured_tile_groups: Dictionary = {
		"tiles": {},
		"collisions": [],
	}
	
	# Combine tiles into rectangular groups to reduce map file size
	var file: FileAccess = FileAccess.open(path + map_name + map_extension, FileAccess.WRITE)
	for id in tile_map.tile_set.get_source_count():
		captured_tile_groups["tiles"][id] = []
		combine_tiles(0, tile_map, id, captured_tile_groups["tiles"][id])
	
	# collisions will only have 1 ID
	combine_tiles(0, collision_map, 1, captured_tile_groups["collisions"])

	# Print out results into a map file
	file.store_line("[header]")
	generate_headers(file)
	file.store_line("\n[tiles]")
	for id in captured_tile_groups["tiles"]:
		for group in captured_tile_groups["tiles"][id]:
			file.store_line(str(id) + ":" + str(group))
	
	file.store_line("\n[collisions]")
	for group in captured_tile_groups["collisions"]:
		file.store_line(str(group))
	
	file.close()
	
func is_directory_good() -> bool:
	var dir = DirAccess.open(path)
	return dir != null

func generate_tiles(layer: int):
	var cells = tile_map.get_used_cells(layer)
	var file: FileAccess = FileAccess.open(path + map_name + map_extension, FileAccess.WRITE)
	
	for cell in cells:
		var cell_id: int = tile_map.get_cell_source_id(layer, cell)
		file.store_line(str(cell) + ":" + str(cell_id))
		
		var tile_data: TileData = tile_map.get_cell_tile_data(layer, cell)
		
	file.close()

# Combine the tiles into vertical strips first
func combine_tiles(layer: int, gen_tile_map: TileMap, id: int, captured_groups: Array):
	var cells = gen_tile_map.get_used_cells_by_id(layer, id)
	
	# Will increment 1 each iteration on y-axis
	cells.sort()

	# Getting vertical groups first
	var groups := []
	var last_cell: Vector2i = cells[0] if cells.size() > 0 else Vector2i.ZERO
	var current_group := [last_cell, last_cell]
	for cell in cells:
		var cell_diff: Vector2i = cell - last_cell
		if abs(cell_diff.y) <= 1:
			current_group[1] = cell
		else:
			groups.append(current_group.duplicate())
			current_group[0] = cell
			current_group[1] = cell
			
		last_cell = cell
	
	# Combing through the rest of the values on the x-axis
	generate_rectangular_tile_groups(groups, captured_groups)

# Combine the strips generated in combine_tiles to rectangles
func generate_rectangular_tile_groups(groups: Array, captured_groups: Array):
	var check_idx: int = 1
	while groups.size() > 0:
		var group_min: Vector2i = groups[0][0]
		var group_max: Vector2i = groups[0][1]
		
		# need this to generate rectangles with width > 2
		var x_axis_iterations: int = 1
		while check_idx < groups.size():
			var next_group_min: Vector2i = groups[check_idx][0]
			var next_group_max: Vector2i = groups[check_idx][1]
			var group_min_diff: Vector2i = next_group_min - group_min
			
			# cannot be a rectangle if this is true
			if group_min_diff.x > x_axis_iterations:
				break
			# keep checking further-along x-axes - skip this index
			elif group_min_diff.x <= (x_axis_iterations-1):
				check_idx += 1
				continue
				
			# now, we know this index has an x value that is 1 greater
			var group_max_diff: Vector2i = next_group_max - group_max
			
			# This can only be a rectangle if the y coordinates match up on min/max
			if group_max_diff.y != 0 || group_min_diff.y != 0:
				check_idx += 1
				continue
			
			# This HAS to be a rectangle now -- update x coordinate
			group_max.x = next_group_max.x
			groups[0][1] = group_max
			groups.pop_at(check_idx)
			
			# Created a valid rect - check next index to see if rect can be wider
			x_axis_iterations += 1

		captured_groups.append(groups.pop_front())
		check_idx = 1

# Goes through the tileset's textures and prints to the format:
# "<texture_id>:<file_path>"
func generate_headers(file: FileAccess):
	for tile_set_id in tile_map.tile_set.get_source_count():
		var atlas: TileSetAtlasSource = tile_map.tile_set.get_source(tile_set_id)
		var dir = DirAccess.open(".") # Get project directory
		var current_dir: String = dir.get_current_dir()
		var last_slash: int = current_dir.rfind("/")
		current_dir = current_dir.substr(last_slash)
		var formatted_path: String = atlas.texture.resource_path.substr(6)
		
		file.store_line(str(tile_set_id) + ":\"" + current_dir + "/" + formatted_path + "\"")
