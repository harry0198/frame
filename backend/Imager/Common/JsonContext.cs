using System.Text.Json.Serialization;
using Imager.Responses;

namespace Imager.Common;

[JsonSerializable(typeof(ImageUploadResponse))]
[JsonSerializable(typeof(ImageResponse))]
public partial class JsonContext : JsonSerializerContext { }