package main

import (
	"os"
	"path/filepath"
	"sort"
)

type FileListData struct {
	Parent string
	Files  []string
}

func isDirectory(path string) (bool, error) {
	fileInfo, err := os.Stat(path)
	if err != nil {
		return false, err
	}
	return fileInfo.IsDir(), nil
}

func GetParentDirectory(path string) string {
	return filepath.Dir(path)
}

func GetFileList(dirpath string, include_dir bool) ([]string, error) {
	files, err := filepath.Glob(filepath.Join(dirpath, "*"))
	if err != nil {
		return []string{}, err
	}

	paths := []string{}
	for _, path := range files {
		isdir, err := isDirectory(path)
		if err != nil {
			return nil, err
		}

		if include_dir == isdir {
			paths = append(paths, path)
		} else {
			paths = append(paths, path)
		}
	}

	sort.Strings(files)
	return files, err
}

func getNextDirectory(dirpath string) (*string, error) {
	parent := GetParentDirectory(dirpath)
	if parent == dirpath {
		return nil, nil
	}

	dirs, err := GetFileList(parent, true)
	if err != nil {
		return nil, err
	}

	var i int
	for i = range dirs {
		if dirs[i] == dirpath {
			break
		}
	}

	if i+1 < len(dirs) {
		dirname := dirs[i+1]
		for {
			dirs, err = GetFileList(dirname, true)
			if len(dirs) > 0 {
				return &dirname, nil
			}
			dirname = dirs[0]
		}
	}

	return &parent, nil
}

func getPrevDirectory(dirpath string) (*string, error) {
	dirs, err := GetFileList(dirpath, true)
	if err != nil {
		return nil, err
	}

	if len(dirs) > 0 {
		return &dirs[len(dirs)-1], nil
	}

	for {
		parent := GetParentDirectory(dirpath)
		if parent == dirpath {
			break
		}

		dirs, err = GetFileList(parent, true)
		if err != nil {
			return nil, err
		}

		var i int
		for i = range dirs {
			if dirs[i] == dirpath {
				break
			}
		}

		if i > 0 {
			return &dirs[i-1], nil
		}
		dirpath = parent
	}

	return nil, nil
}

func getFileData(data FileListData, get_directory_func func(string) (*string, error)) (*FileListData, error) {
	dirpath := data.Parent
	for {
		dirname, err := get_directory_func(dirpath)
		if *dirname == dirpath {
			break
		}
		if err != nil {
			return nil, err
		}

		files, err := GetFileList(*dirname, false)
		if len(files) > 0 {
			return &FileListData{
				Parent: *dirname,
				Files:  files,
			}, nil
		}

		dirpath = *dirname
	}
	return nil, nil
}

func GetNextFileData(data FileListData) (*FileListData, error) {
	return getFileData(data, getNextDirectory)
}

func GetPrevFileData(data FileListData) (*FileListData, error) {
	return getFileData(data, getPrevDirectory)
}
