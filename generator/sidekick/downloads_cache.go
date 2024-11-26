// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import (
	"crypto/sha256"
	"fmt"
	"io"
	"net/http"
	"os"
	"path"
	"path/filepath"
	"strings"
	"time"

	"github.com/walle/targz"
)

func makeGoogleapisRoot(rootConfig *Config) (string, error) {
	googleapisRoot, ok := rootConfig.Source["googleapis-root"]
	if !ok {
		return "", nil
	}
	if ok := isDirectory(googleapisRoot); ok {
		return googleapisRoot, nil
	}
	if !requiresDownload(googleapisRoot) {
		return "", fmt.Errorf("only directories and https URLs are supported for googleapis-root")
	}
	// Treat `googleapis-root` as a URL to download. We want to avoid downloads
	// if possible, so we will first try to use a cache directory in $HOME.
	// Only if that fails we try a new download.
	googleapisSha256, ok := rootConfig.Source["googleapis-sha256"]
	if !ok {
		return "", fmt.Errorf("using an https:// URL for googleapis-root requires setting googleapis-sha256")
	}
	cacheDir, err := getCacheDir(rootConfig)
	if err != nil {
		return "", err
	}
	target := path.Join(cacheDir, googleapisSha256)
	if isDirectory(target) {
		return target, nil
	}
	tgz := target + ".tar.gz"
	if err := downloadGoogleapisRoot(tgz, googleapisRoot, googleapisSha256); err != nil {
		return "", err
	}

	if err := targz.Extract(tgz, cacheDir); err != nil {
		return "", err
	}
	name := extractedName(rootConfig, googleapisRoot)
	if err := os.Rename(path.Join(cacheDir, name), target); err != nil {
		return "", err
	}
	return target, nil
}

func extractedName(rootConfig *Config, googleapisRoot string) string {
	name, ok := rootConfig.Source["googleapis-extracted-name"]
	if ok {
		return name
	}
	return "googleapis-" + filepath.Base(strings.TrimSuffix(googleapisRoot, ".tar.gz"))
}

func downloadGoogleapisRoot(target, source, sha256 string) error {
	if fileExists(target) {
		return nil
	}
	var err error
	backoff := 10 * time.Second
	for i := range 3 {
		if i != 0 {
			time.Sleep(backoff)
			backoff = 2 * backoff
		}
		if err = downloadAttempt(target, source, sha256); err == nil {
			return nil
		}
	}
	return fmt.Errorf("download failed after 3 attempts, last error=%w", err)
}

func downloadAttempt(target, source, expectedSha256 string) error {
	if err := os.MkdirAll(filepath.Dir(target), 0777); err != nil {
		return err
	}
	tempFile, err := os.CreateTemp(filepath.Dir(target), "temp-")
	if err != nil {
		return err
	}
	defer os.Remove(tempFile.Name())

	response, err := http.Get(source)
	if err != nil {
		return err
	}
	if response.StatusCode >= 300 {
		return fmt.Errorf("http error in download %s", response.Status)
	}

	if _, err := io.Copy(tempFile, response.Body); err != nil {
		return err
	}
	if err := tempFile.Close(); err != nil {
		return err
	}
	if err := response.Body.Close(); err != nil {
		return err
	}
	file, err := os.Open(tempFile.Name())
	if err != nil {
		return err
	}
	hasher := sha256.New()
	if _, err := io.Copy(hasher, file); err != nil {
		return err
	}
	got := fmt.Sprintf("%x", hasher.Sum(nil))
	if expectedSha256 != got {
		return fmt.Errorf("mismatched hash on download, expected=%s, got=%s", expectedSha256, got)
	}
	return os.Rename(tempFile.Name(), target)
}

func fileExists(name string) bool {
	stat, err := os.Stat(name)
	if err != nil {
		return false
	}
	return stat.Mode().IsRegular()
}

func isDirectory(name string) bool {
	stat, err := os.Stat(name)
	if err != nil {
		return false
	}
	if !stat.IsDir() {
		return false
	}
	return true
}

func getCacheDir(rootConfig *Config) (string, error) {
	cacheDir, ok := rootConfig.Source["cachedir"]
	if !ok {
		var err error
		if cacheDir, err = os.UserCacheDir(); err != nil {
			return "", err
		}
	}
	return path.Join(cacheDir, "sidekick", "downloads"), nil
}

func requiresDownload(googleapisRoot string) bool {
	return strings.HasPrefix(googleapisRoot, "https://") || strings.HasPrefix(googleapisRoot, "http://")
}