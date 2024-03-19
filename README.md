<div align="center">
  <h1>Whisper CLI</h1>
  <p>
    <b>
      Transcribe media files from the terminal.
    </b>
  </p>
  <sub>
    Built on top of
    <a href="https://github.com/ggerganov/whisper.cpp" target="_blank">whisper.cpp</a>.
  </sub>
</div>

## Abstract

Transcribing with [`whisper.cpp`](https://github.com/ggerganov/whisper.cpp) is awesome, but
sometimes cumbersome when dealing with different input types.

Whisper CLI makes it easier by removing the type constraints; under the hood it uses
[`ffmpeg`](https://ffmpeg.org/) to covnert the input data into a 16000 Hz mono wave file,
ready to be ingested by `whisper.cpp`. After transcription, the converted file is
automatically removed.

## Requirements

Whisper CLI uses your system's `git`, `clang` and `ffmpeg`; make sure you have them installed.

## License

_Whisper CLI_ is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
