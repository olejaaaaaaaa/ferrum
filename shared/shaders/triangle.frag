#version 450
layout(location = 0) in vec3 fragColor;
layout(location = 0) out vec4 outColor;

layout(location = 1) in float iTime;
layout(binding = 1) uniform sampler2D tex; // Правильное объявление sampler2D

void main() {
    // Генерируем UV из координат экрана (для полноэкранного эффекта)
    vec2 uv = gl_FragCoord.xy / vec2(800.0, 600.0); // Подставьте свои размеры окна
    
    // Ваш эффект nebula
    vec3 nebula = vec3(
        0.5 + 0.5 * sin(iTime + fragColor.r * 5.0),
        0.5 + 0.5 * cos(iTime * 1.3 + fragColor.g * 3.0),
        0.5 + 0.5 * sin(iTime * 0.7 + fragColor.b * 4.0)
    );
    
    // Чтение из текстуры с использованием sampler2D
    vec4 textureColor = texture(tex, uv);
    
    // Комбинируем nebula с текстурой (пример комбинации)
    outColor = vec4(nebula * textureColor.rgb, 1.0);
}