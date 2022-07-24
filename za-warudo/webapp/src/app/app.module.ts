import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule } from '@angular/forms';

import { AppComponent } from './app.component';
import { MonacoEditorModule } from 'ngx-monaco-editor';

@NgModule({
  declarations: [AppComponent],
  imports: [BrowserModule, FormsModule, MonacoEditorModule.forRoot()],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
