import * as React from 'react';
import { I18n } from '@lingui/react';
import { t } from '@lingui/macro';
import { faExternalLinkAlt } from '@fortawesome/free-solid-svg-icons';
import { IconButton } from '../elements';
import * as proto from '../proto';
import { bookToString } from '../util/proto';

export type Props = {
  translation: proto.instantbible.data.Translation;
  verseKey: proto.instantbible.data.IVerseKey;
} & Pick<React.HTMLProps<HTMLButtonElement>, 'className'>;

const { Translation } = proto.instantbible.data;

const makeUrl = (
  translation: proto.instantbible.data.Translation,
  verseKey: proto.instantbible.data.IVerseKey,
) => {
  const book = bookToString(verseKey.book);
  const chapter = verseKey.chapter;

  if (translation === Translation.BSB) {
    return `https://biblehub.com/bsb/${book}/${chapter}.htm`;
  }

  if (translation === Translation.NET) {
    return `https://netbible.org/bible/${book}+${chapter}`;
  }

  if (translation === Translation.KJV) {
    return `https://www.biblegateway.com/passage/?search=${book}+${chapter}&version=KJV`;
  }

  return null;
};

const OpenExternalButton = ({ className, translation, verseKey }: Props) => {
  const handleOpen = React.useCallback(() => {
    const url = makeUrl(translation, verseKey);
    if (url) {
      window.open(url, '_blank', 'noopener');
    }
  }, [translation, verseKey]);

  return (
    <I18n>
      {({ i18n }) => (
        <IconButton
          className={className}
          icon={faExternalLinkAlt}
          onClick={handleOpen}
          title={i18n._(t`Open in Bible`)}
        />
      )}
    </I18n>
  );
};

export default React.memo(OpenExternalButton);
