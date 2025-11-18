import type { ImageInfo } from "./types";

const API_BASE_URL = `http://192.168.0.40:5000`;

export async function getImages() {
    const response = await fetch(`${API_BASE_URL}/images`);
    if (!response.ok) {
        throw new Error(`Network response was not ok, received status ${response.status}`);
    }
    
    const data: ImageInfo[] = await response.json();
    return data.map(item => ({
        url: `${API_BASE_URL}${item.url}`,
        filePath: item.filePath,
        fileNameWithExtension: item.fileNameWithExtension
    }));
}

export async function uploadFile(file: File): Promise<void> {
  const formData = new FormData();
  formData.append("file", file);

  const response = await fetch(`${API_BASE_URL}/upload`, {
    method: "POST",
    body: formData,
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Upload failed: ${response.status} ${errorText}`);
  }
}

export async function deleteImage(image: string): Promise<void> {
    await fetch(`${API_BASE_URL}/images/${image}`, {
        method: 'DELETE',
    });
}

export async function displayImageNow(image: string): Promise<void> {
  const body  = {
    filePath: image,
  }
  
  const response = await fetch(`${API_BASE_URL}/inky`, {
    method: "POST",
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Display failed: ${response.status} ${errorText}`);
  }
}