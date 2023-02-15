struct VSInput {
	@location(0) position: vec4<f32>, 
	@location(1) color: vec4<f32>, 
}

struct VSOutput {
	@builtin(position) clip_position: vec4<f32>, 
	@location(0) color: vec4<f32>, 
}

@vertex
fn vs_main(
	model: VSInput, 
) -> VSOutput {
	var out = VSOutput {
		clip_position: model.position, 
		color: model.color, 
	};
	return out;
}

@fragment
fn fs_main(
	in: VSOutput, 
) -> @location(0) vec4<f32> {
	return in.color;
}