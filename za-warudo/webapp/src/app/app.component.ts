import { Component, ElementRef, ViewChild } from '@angular/core';

import * as tumult from 'tumult';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  noise: any;

  @ViewChild('canvas') canvas!: ElementRef<HTMLCanvasElement>;

  constructor() {
    this.noise = new tumult.Simplex2();
  }

  ngOnInit() {
  }

  ngAfterViewInit() {
    console.log("aaa");
    const canvas = this.canvas.nativeElement;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      console.log("aaa no ctx", ctx);
      return;
    }
    const image = ctx.createImageData(canvas.width, canvas.height);
    let data = image.data;

    for (let i = 0; i < canvas.width; i++) {
      for (let j = 0; j < canvas.height; j++) {
        const x = (i / canvas.width) * 4;
        const y = (j / canvas.height) * 4;
        const noise = (this.noise.gen(x, y) + 1) / 2;
        const index = (j * canvas.width + i) * 4;
        data[index] = noise * 255;
        data[index + 1] = noise * 255;
        data[index + 2] = noise * 255;
        data[index + 3] = 255;
      }
    }

    // ctx.fillStyle = 'black';
    // ctx.fillRect(0, 0, 100, 100);
    ctx.putImageData(image, 0, 0);
  }
}
