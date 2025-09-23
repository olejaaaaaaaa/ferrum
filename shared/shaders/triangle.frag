#version 450
layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragUV;
layout(location = 2) in vec3 fragNormal;
layout(location = 3) in vec3 fragWorldPos;

layout(location = 0) out vec4 outColor;

const vec3 LIGHT_DIR = normalize(vec3(1.0, 1.0, 1.0));
const vec3 LIGHT_COLOR = vec3(1.0, 1.0, 1.0);
const vec3 AMBIENT = vec3(0.1);

void main() {

    vec3 normal = normalize(fragNormal);

    float diff = max(dot(normal, LIGHT_DIR), 0.0);
    vec3 diffuse = diff * LIGHT_COLOR * fragColor;

    vec3 result = AMBIENT * fragColor + diffuse;

    outColor = vec4(result, 1.0);
}