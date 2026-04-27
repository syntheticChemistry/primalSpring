## godot_bridge.gd — Example Godot 4 GDScript for petalTongue framebuffer bridge
##
## Attach this to a Node in your Godot scene. Each frame (or at a configurable
## interval), it captures the viewport as raw RGBA8 bytes and writes them to a
## shared-memory file that godot_bridge.sh watches and uploads to petalTongue.
##
## Setup:
##   1. Copy this script into your Godot project
##   2. Attach to any Node (e.g. root or a dedicated "Bridge" node)
##   3. Run godot_bridge.sh alongside (it watches the SHM file)
##   4. petalTongue displays the Godot viewport as a Primitive::Texture
##
## The bridge is one-directional (Godot → petalTongue). For input feedback
## from petalTongue back to Godot, use interaction.poll JSON-RPC.

extends Node

## Path to the shared memory file for frame data.
@export var shm_path: String = "/dev/shm/godot-frame.rgba"
## Path to the metadata file (resolution, format).
@export var meta_path: String = "/dev/shm/godot-frame.meta"
## How often to capture frames (0 = every frame).
@export var capture_interval_ms: int = 16
## Whether the bridge is active.
@export var active: bool = true

var _last_capture_time: int = 0
var _frame_count: int = 0
var _file := FileAccess

func _ready() -> void:
	_write_meta()
	print("[godot_bridge] Bridge active — writing to: ", shm_path)

func _process(_delta: float) -> void:
	if not active:
		return

	var now := Time.get_ticks_msec()
	if now - _last_capture_time < capture_interval_ms:
		return
	_last_capture_time = now

	_capture_and_write()

func _capture_and_write() -> void:
	var viewport := get_viewport()
	if viewport == null:
		return

	var image := viewport.get_texture().get_image()
	if image == null:
		return

	# Ensure RGBA8 format for the bridge
	if image.get_format() != Image.FORMAT_RGBA8:
		image.convert(Image.FORMAT_RGBA8)

	var data := image.get_data()

	# Write raw pixels to shared memory
	var file := FileAccess.open(shm_path, FileAccess.WRITE)
	if file == null:
		push_warning("[godot_bridge] Cannot open SHM file: " + shm_path)
		return
	file.store_buffer(data)
	file.close()

	_frame_count += 1
	if _frame_count % 60 == 0:
		print("[godot_bridge] Captured ", _frame_count, " frames")

func _write_meta() -> void:
	var viewport := get_viewport()
	if viewport == null:
		return

	var size := viewport.get_visible_rect().size
	var file := FileAccess.open(meta_path, FileAccess.WRITE)
	if file == null:
		return
	file.store_string("GODOT_WIDTH=%d\nGODOT_HEIGHT=%d\n" % [int(size.x), int(size.y)])
	file.close()
