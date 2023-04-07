uniform vec3 cameraPosition;
uniform sampler3D tex;
uniform vec3 size;

in vec3 pos;

layout (location = 0) out vec4 outColor;

void main() {
    int steps = 200;
    vec3 rayDir = normalize(pos - cameraPosition);
    const float minDistFromCamera = 0.2;
    vec3 rayPos = cameraPosition + minDistFromCamera * rayDir;
    float stepSize = length(size) / float(steps);
    vec3 step = rayDir * stepSize;
    vec4 accumulatedColor = vec4(0.0);

    for (int i = 0; i < steps; i++) {
        if (i == steps - 1 || accumulatedColor.a >= 1.0) {
            break;
        }

        if (rayPos.x < -0.5 * size.x || rayPos.y < -0.5 * size.y || rayPos.z < -0.5 * size.z ||
            rayPos.x > 0.5 * size.x || rayPos.y > 0.5 * size.y || rayPos.z > 0.5 * size.z) {
            break;
        }

        vec3 uvw = (rayPos / size) + 0.5;
        vec4 sampleColor = texture(tex, uvw);
        sampleColor.a *= stepSize;

        accumulatedColor.rgb += (1.0 - accumulatedColor.a) * sampleColor.rgb * sampleColor.a;
        accumulatedColor.a += (1.0 - accumulatedColor.a) * sampleColor.a;

        rayPos += step;
    }

    outColor = accumulatedColor;
}