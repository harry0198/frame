namespace Imager.Api.Validation;

public static class ImageValidation
{
    private static readonly Dictionary<SupportedFileType, List<byte[]>> FileSignatures = 
            new()
            {
                {
                    SupportedFileType.Jpeg,
                    [
                        new byte[] { 0xFF, 0xD8, 0xFF, 0xE0 },
                        new byte[] { 0xFF, 0xD8, 0xFF, 0xE2 },
                        new byte[] { 0xFF, 0xD8, 0xFF, 0xE3 },
                    ]
                },
                {
                    SupportedFileType.Png,
                    [
                        new byte[] { 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A }
                    ]
                },
                {
                    SupportedFileType.Heic,
                    [
                        new byte[] { 0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x69, 0x63 },
                        new byte[] { 0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x69, 0x78 },
                        new byte[] { 0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x76, 0x63 },
                        new byte[] { 0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x76, 0x78 }
                    ]
                }
            };

    public static async Task<bool> IsValidImageSignature(IFormFile formFile, SupportedFileType fileType, CancellationToken cancellation)
    {
        var maxHeaderSize = FileSignatures[fileType].Max(s => s.Length);
        var headerBytes = new byte[maxHeaderSize];

        await using var input = formFile.OpenReadStream();
        await input.ReadExactlyAsync(headerBytes.AsMemory(0, maxHeaderSize), cancellation);

        var signatures = FileSignatures[fileType];
        return signatures.Any(signature =>
            headerBytes.Take(signature.Length).SequenceEqual(signature));
    }
}