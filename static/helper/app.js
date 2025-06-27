// Tab switching functionality
document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        
        tab.classList.add('active');
        document.getElementById(tab.dataset.tab).classList.add('active');
    });
});

// Algorithm selection
document.querySelectorAll('.crypto-card').forEach(card => {
    card.addEventListener('click', () => {
        const container = card.parentElement;
        container.querySelectorAll('.crypto-card').forEach(c => c.classList.remove('selected'));
        card.classList.add('selected');
    });
});

// File drop zone functionality
function setupDropZone(dropZoneId, fileInputId, fileListId) {
    const dropZone = document.getElementById(dropZoneId);
    const fileInput = document.getElementById(fileInputId);
    const fileList = document.getElementById(fileListId);

    dropZone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropZone.classList.add('dragover');
    });

    dropZone.addEventListener('dragleave', () => {
        dropZone.classList.remove('dragover');
    });

    dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.classList.remove('dragover');
        const files = e.dataTransfer.files;
        handleFiles(files, fileListId);
    });

    fileInput.addEventListener('change', (e) => {
        handleFiles(e.target.files, fileListId);
    });
}

function handleFiles(files, fileListId) {
    const fileList = document.getElementById(fileListId);
    fileList.innerHTML = '';

    Array.from(files).forEach(file => {
        const fileItem = document.createElement('div');
        fileItem.className = 'file-item';
        
        const isLarge = file.size > 50 * 1024 * 1024; // 50MB threshold for streaming
        
        fileItem.innerHTML = `
            <div class="file-info">
                <div class="file-name">${file.name}</div>
                <div class="file-size">${formatFileSize(file.size)} ${isLarge ? '(Streaming enabled)' : ''}</div>
            </div>
            <div class="file-status status-pending">Pending</div>
        `;
        
        fileList.appendChild(fileItem);
    });

    // Show streaming indicator for large files
    const hasLargeFiles = Array.from(files).some(file => file.size > 50 * 1024 * 1024);
    const streamingIndicator = document.querySelector('.streaming-indicator');
    if (streamingIndicator) {
        streamingIndicator.style.display = hasLargeFiles ? 'inline-flex' : 'none';
    }
}

function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// Setup all drop zones
setupDropZone('encryptDropZone', 'encryptFiles', 'encryptFileList');
setupDropZone('decryptDropZone', 'decryptFiles', 'decryptFileList');
setupDropZone('batchDropZone', 'batchFiles', 'batchFileList');

// Form submission handlers
document.getElementById('encryptForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const password = document.getElementById('encryptPassword').value;
    const confirmPassword = document.getElementById('encryptConfirm').value;
    
    if (password !== confirmPassword) {
        alert('Passwords do not match!');
        return;
    }
    
    const files = document.getElementById('encryptFiles').files;
    const algorithm = document.querySelector('.crypto-card.selected').dataset.algo;
    console.log(algorithm)
    console.log(password)
     const formData = new FormData();
      formData.append("algorithm", algorithm);
      formData.append("password", password);
      formData.append("file", files[0]);

        const response = await fetch('/api/encrypt', {
            method: 'POST',
            body: formData
        });
        const data = await response.json(); // parses JSON

        console.log("✅ Success:", data.success);
        console.log("💬 Message:", data.message);
        console.log("🆔 File ID:", data.file_id);
        console.log("📦 Encrypted Data (Base64):", data.encrypted_data);
});

document.getElementById('decryptForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const password = document.getElementById('decryptPassword').value;
    const files = document.getElementById('decryptFiles').files;
    
    await processFiles('decrypt', files, password);
});

document.getElementById('batchForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const password = document.getElementById('batchPassword').value;
    const files = document.getElementById('batchFiles').files;
    const algorithm = document.getElementById('batchAlgorithm').value;
    const operation = document.querySelector('#batch .crypto-card.selected').dataset.operation;
    
    await processFiles(operation, files, password, algorithm, true);
});

async function processFiles(operation, files, password, algorithm = 'aes', isBatch = false) {
    const prefix = isBatch ? 'batch' : operation;
    const progressContainer = document.getElementById(`${prefix}Progress`);
    const progressFill = progressContainer.querySelector('.progress-fill');
    const progressText = progressContainer.querySelector('.progress-text');
    const submitBtn = document.getElementById(`${prefix}Btn`);
    
    // Show progress and disable button
    progressContainer.style.display = 'block';
    submitBtn.disabled = true;
    submitBtn.textContent = 'Processing...';
    submitBtn.classList.add('processing');
    
    // Update file statuses
    const fileItems = document.querySelectorAll(`#${prefix}FileList .file-item`);
    
    try {
        const formData = new FormData();
        formData.append('operation', operation);
        formData.append('password', password);
        formData.append('algorithm', algorithm);
        
        Array.from(files).forEach(file => {
            formData.append('files', file);
        });
        
        // Simulate progress for demo
        let progress = 0;
        const progressInterval = setInterval(() => {
            progress += Math.random() * 15;
            if (progress > 90) progress = 90;
            
            progressFill.style.width = progress + '%';
            progressText.textContent = `Processing... ${Math.round(progress)}%`;
            
            // Update file statuses
            fileItems.forEach((item, index) => {
                const status = item.querySelector('.file-status');
                if (progress > (index + 1) * (90 / fileItems.length)) {
                    status.textContent = 'Processing';
                    status.className = 'file-status status-processing';
                }
            });
        }, 200);
        
        // Make API call to your Rust backend
        const response = await fetch('/api/process', {
            method: 'POST',
            body: formData
        });
        
        clearInterval(progressInterval);
        
        if (response.ok) {
            progressFill.style.width = '100%';
            progressText.textContent = 'Complete!';
            
            // Update all file statuses to complete
            fileItems.forEach(item => {
                const status = item.querySelector('.file-status');
                status.textContent = 'Complete';
                status.className = 'file-status status-complete';
            });
            
            // Show download section
            showDownloadSection(await response.json());
        } else {
            throw new Error('Processing failed');
        }
        
    } catch (error) {
        console.error('Error:', error);
        progressText.textContent = 'Error occurred';
        
        // Update file statuses to error
        fileItems.forEach(item => {
            const status = item.querySelector('.file-status');
            status.textContent = 'Error';
            status.className = 'file-status status-error';
        });
        
        alert('An error occurred during processing. Please try again.');
    } finally {
        submitBtn.disabled = false;
        submitBtn.textContent = operation === 'encrypt' ? '🔒 Encrypt Files' : 
                                operation === 'decrypt' ? '🔓 Decrypt Files' : '📦 Process Batch';
        submitBtn.classList.remove('processing');
    }
}

function showDownloadSection(data) {
    const downloadSection = document.getElementById('downloadSection');
    const downloadLinks = document.getElementById('downloadLinks');
    
    downloadLinks.innerHTML = '';
    
    if (data.files) {
        data.files.forEach(file => {
            const downloadBtn = document.createElement('button');
            downloadBtn.className = 'btn download-btn';
            downloadBtn.textContent = `📥 ${file.name}`;
            downloadBtn.onclick = () => downloadFile(file.url, file.name);
            downloadLinks.appendChild(downloadBtn);
        });
    }
    
    downloadSection.style.display = 'block';
}

function downloadFile(url, filename) {
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
}

// Password strength indicator
document.getElementById('encryptPassword').addEventListener('input', (e) => {
    // You can add password strength validation here
    const password = e.target.value;
    // Add visual feedback for password strength
});

// Real-time password matching
document.getElementById('encryptConfirm').addEventListener('input', (e) => {
    const password = document.getElementById('encryptPassword').value;
    const confirm = e.target.value;
    
    if (confirm && password !== confirm) {
        e.target.style.borderColor = '#ef4444';
    } else {
        e.target.style.borderColor = '#e5e7eb';
    }
});