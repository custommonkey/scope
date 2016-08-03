#version 140

out vec4 color;

uniform sampler2D fb;

vec2 iResolution = vec2(1024, 768);

float normpdf(in float x, in float sigma) {
	return 0.39894 * exp(-0.5 * x * x / (sigma * sigma)) / sigma;
}

const int mSize = 11;
const int kSize = (mSize-1)/2;
const float sigma = 7.0;

void main() {

	vec3 c = texture(fb, gl_FragCoord.xy).rgb;

	//declare stuff
	vec3 final_colour = vec3(0.0);

	//create the 1-D kernel
	float Z = 0.0;

	float kernel[mSize];

	for (int j = 0; j <= kSize; ++j) {
		kernel[kSize+j] = kernel[kSize-j] = normpdf(float(j), sigma);
	}

	//get the normalization factor (as the gaussian has been clamped)
	for (int j = 0; j < mSize; ++j) {
		Z += kernel[j];
	}

	vec3 back = vec3(0.9, 0.95, 0.9);

	//read out the texels
	for (int i =- kSize; i <= kSize; ++i) {
		for (int j =- kSize; j <= kSize; ++j) {
			final_colour += kernel[kSize+j]
	            * kernel[kSize + i]
	            * texture(fb, (gl_FragCoord.xy + vec2(float(i), float(j))) / iResolution.xy).rgb
	            * back
	            ;
		}
	}

	color = vec4(final_colour/(Z*Z), 1.0);

}
