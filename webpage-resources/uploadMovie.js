document.getElementById('uploadVideoBtn').addEventListener('click', function() {
    const videoFile = document.getElementById('videoFile').files[0];
    if (videoFile) {
        uploadVideo(videoFile);
    } else {
        alert('Please select a video file to upload.');
    }
});

async function uploadVideo(file) {
    const formData = new FormData();
    formData.append('videoFile', file);

    try {
        const uploadResponse = await fetch('/upload-video', {
            method: 'POST',
            body: formData
        });

        if (!uploadResponse.ok) {
            throw new Error('Upload failed');
        }

        const uploadData = await uploadResponse.json();
        const { filename, duration, title } = uploadData;

        console.log('Video duration:', duration);
        console.log('Video title:', title);

        // Call the WebAssembly function (if needed)
        // process_video(videoFile);

        // Delete the video file after processing
        const deleteResponse = await fetch(`/delete-video/${encodeURIComponent(filename)}`, {
            method: 'DELETE'
        });

        if (!deleteResponse.ok) {
            throw new Error('Failed to delete video');
        }

    } catch (error) {
        console.error('Error uploading or processing video:', error);
    }
}
